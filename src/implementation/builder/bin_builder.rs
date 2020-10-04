use core::mem;

use serde::export::PhantomData;
use smallvec::SmallVec;

use crate::{AnyBin, BinBuilder, BinSegment, Factory, SBin, Segment, StackBinBuilder};

/// There's two things we want to optimize:
///
///  * Only one single segment: Say for example if you add just a static-bin, we want to make
/// sure there's no allocation.
///  * Small length: Length that can be stack-allocated should not allocate.
///  * When there's larger items, we want to know the entire length when building (so
/// we know how many bytes to allocate for the vector).
///
/// ... for all other cases it's OK to allocate a `Vec<u8>` (since when converting to `Bin`
/// this `Vec<u8>` must be allocated anyways).
pub struct DefaultBinBuilder<'a, TFactory: Factory, TConfig> {
    state: State<'a, TFactory::T>,
    _phantom: PhantomData<TConfig>,
}

pub trait BuilderCfg<TAnyBin: AnyBin> {
    fn convert_from_sbin_to_t(sbin: SBin) -> TAnyBin;
    fn vec_excess_capacity() -> usize;
}

/// Unfortunately, a segment is quite large, so we can't have too many on the stack. Each segment
/// is takes about 40 bytes (on 64 bit machines).
const SMALL_VEC_MAX_SEGMENTS: usize = 8;

impl<'a, TFactory, TConfig> BinBuilder<'a> for DefaultBinBuilder<'a, TFactory, TConfig> where TFactory: Factory, TConfig: BuilderCfg<TFactory::T> {
    type T = TFactory::T;

    #[inline]
    fn push(&mut self, segment: impl Into<BinSegment<'a, Self::T>>) {
        let segment = segment.into();
        // finish here, since that's a no-op.
        if segment.is_empty() {
            return;
        }

        match &mut self.state {
            State::State0Empty => {
                self.state = State::State1Single(segment)
            }
            State::State1Single(single) => {
                let single = mem::replace(single, BinSegment::Empty);

                // if both items fit onto the stack, we go to state 2...
                let stack_builder = {
                    let mut stack_builder = StackBinBuilder::new(0);
                    let fits_onto_stack = stack_builder.try_extend_from_slice(single.as_slice());
                    if !fits_onto_stack {
                        None
                    } else {
                        let fits_onto_stack = stack_builder.try_extend_from_slice(segment.as_slice());
                        if !fits_onto_stack {
                            None
                        } else {
                            Some(stack_builder)
                        }
                    }
                };
                if let Some(stack_builder) = stack_builder {
                    // ok, small enough, go to state 2.
                    self.state = State::State2Stack(stack_builder);
                } else {
                    // nope, they're large, go to state 3
                    let mut segments = SegmentsSmallVec::new();
                    let number_of_bytes = single.number_of_bytes()
                        .checked_add(segment.number_of_bytes()).unwrap();
                    segments.push(single);
                    segments.push(segment);

                    self.state = State::State3Large {
                        segments,
                        number_of_bytes,
                    }
                }
            }
            State::State2Stack(stack_builder) => {
                let fits_onto_stack = stack_builder.try_extend_from_slice(segment.as_slice());
                if fits_onto_stack {
                    // nice! keep state 2
                } else {
                    // unfortunately we have to go to state 3...
                    let sbin = stack_builder.build_stack_only().expect("Implementation \
                    error: We made sure that the stack builder does not grow too large.");

                    let mut segments = SegmentsSmallVec::new();
                    let number_of_bytes = sbin.len()
                        .checked_add(segment.number_of_bytes()).unwrap();
                    segments.push(BinSegment::Bin(TConfig::convert_from_sbin_to_t(sbin)));
                    segments.push(segment);

                    self.state = State::State3Large {
                        segments,
                        number_of_bytes,
                    }
                }
            }
            State::State3Large { segments, number_of_bytes } => {
                let new_number_of_bytes = (*number_of_bytes)
                    .checked_add(segment.number_of_bytes()).unwrap();
                *number_of_bytes = new_number_of_bytes;
                segments.push(segment);
                // keep state 3
            }
        }
    }

    fn build(&mut self) -> Self::T {
        // builder will be empty after this call
        let taken_state = mem::replace(&mut self.state, State::State0Empty);
        match taken_state {
            State::State0Empty => TFactory::empty(),
            State::State1Single(single) => TFactory::from_segment(single),
            State::State2Stack(stack_builder) => {
                let sbin = stack_builder.build_stack_only().expect("Implementation \
                    error: We made sure that the stack builder does not grow too large.");
                TConfig::convert_from_sbin_to_t(sbin)
            }
            State::State3Large { segments, number_of_bytes } => {
                // allocate a vector that's large enough
                let mut vec = Vec::with_capacity(
                    TConfig::vec_excess_capacity().checked_add(number_of_bytes).unwrap());
                for segment in segments {
                    vec.extend_from_slice(segment.as_slice());
                }
                TFactory::from_given_vec(vec)
            }
        }
    }
}

type SegmentsSmallVec<'a, TAnyBin> = SmallVec<[BinSegment<'a, TAnyBin>; SMALL_VEC_MAX_SEGMENTS]>;

enum State<'a, TAnyBin: AnyBin> {
    /// initial state. Nothing in builder.
    State0Empty,
    /// one single item in builder.
    State1Single(BinSegment<'a, TAnyBin>),
    /// Multiple items in builder that all fit onto the stack.
    State2Stack(StackBinBuilder),
    /// multiple items in builder that are too large for the stack.
    State3Large {
        segments: SegmentsSmallVec<'a, TAnyBin>,
        number_of_bytes: usize,
    },
}