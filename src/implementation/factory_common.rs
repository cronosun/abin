use crate::{
    AnyBin, AnyRc, ArcBin, Bin, BinSegment, DefaultExcessShrink, DefaultGivenVecConfig,
    EmptyBin, ExcessShrink, Factory, GivenVecConfig, GivenVecOptimization, IntoUnSyncView,
    maybe_shrink, New, RcBin, SBin, SegmentsIterator, SNew, StackBin, StackBinBuilder, StaticBin,
    VecBin,
};

pub trait CommonFactory {
    type TAnyRc: AnyRc;
    type TFunctions: CommonFactoryFunctions<TSync=SBin, TUnSync=<Self::TAnyRc as AnyRc>::T>;
}

pub trait CommonFactoryFunctions {
    type TSync;
    type TUnSync;
    fn convert_to_un_sync(value: Self::TSync) -> Self::TUnSync;
    fn is_sync() -> bool;
}

impl<TCf> Factory for TCf
    where
        TCf: CommonFactory,
        <TCf::TAnyRc as AnyRc>::T: AnyBin,
{
    type T = <TCf::TAnyRc as AnyRc>::T;

    #[inline]
    fn empty() -> Self::T {
        TCf::TFunctions::convert_to_un_sync(EmptyBin::new())
    }

    #[inline]
    fn from_static(slice: &'static [u8]) -> Self::T {
        TCf::TFunctions::convert_to_un_sync(StaticBin::from(slice))
    }

    #[inline]
    fn copy_from_slice(slice: &[u8]) -> Self::T {
        // use stack if it's small
        if let Some(stack_bin) = StackBin::try_from(slice) {
            TCf::TFunctions::convert_to_un_sync(stack_bin)
        } else {
            TCf::TAnyRc::copy_from_slice(slice)
        }
    }

    #[inline]
    fn from_iter(iter: impl IntoIterator<Item=u8>) -> Self::T {
        let iter = iter.into_iter();
        match StackBin::try_from_iter(iter) {
            Ok(stack) => TCf::TFunctions::convert_to_un_sync(stack),
            Err(iterator) => TCf::TAnyRc::from_iter(iterator),
        }
    }

    #[inline]
    fn from_iter_new_with_config<'a, T: GivenVecConfig, TIterator>(
        iter: TIterator,
    ) -> Self::T where TIterator: SegmentsIterator<'a, Self::T> {
        if iter.is_empty() {
            TCf::TFunctions::convert_to_un_sync(EmptyBin::new())
        } else {
            match iter.single() {
                Ok(single) => {
                    // nice, one single item
                    Self::from_segment_with_config::<T>(single)
                }
                Err(iter) => {
                    // no, we have multiple segments ... would be nice if length is known
                    match iter.exact_number_of_bytes() {
                        (Some(number_of_bytes)) if number_of_bytes > StackBin::max_len() => {
                            // to long for stack ... collect into a vec (at least we know the exact capacity).
                            let mut vec: Vec<u8> =
                                Vec::with_capacity(number_of_bytes + Self::vec_excess());
                            for item in iter {
                                vec.extend_from_slice(item.as_slice());
                            }
                            Self::from_given_vec_with_config::<T>(vec)
                        }
                        _ => {
                            // we don't know the length or it's small enough for stack (both cases
                            // share the same code).
                            let mut stack_builder = StackBinBuilder::new(Self::vec_excess());
                            for item in iter {
                                stack_builder.extend_from_slice(item.as_slice());
                            }
                            match stack_builder.build() {
                                Ok(stack_bin) => TCf::TFunctions::convert_to_un_sync(stack_bin),
                                Err(vec) => {
                                    // was too large for the stack
                                    Self::from_given_vec_with_config::<T>(vec)
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    #[inline]
    fn from_iter_new<'a, TIterator>(
        iter: TIterator,
    ) -> Self::T where TIterator: SegmentsIterator<'a, Self::T> {
        Self::from_iter_new_with_config::<'a, DefaultGivenVecConfig, TIterator>(iter)
    }

    #[inline]
    fn from_segment_with_config<T: GivenVecConfig>(segment: BinSegment<Self::T>) -> Self::T {
        match segment {
            BinSegment::Slice(slice) => Self::copy_from_slice(slice),
            BinSegment::Static(slice) => Self::from_static(slice),
            BinSegment::Bin(bin) => bin,
            BinSegment::GivenVec(vec) => Self::from_given_vec_with_config::<T>(vec),
            BinSegment::Empty => Self::empty(),
        }
    }

    #[inline]
    fn from_segment(segment: BinSegment<Self::T>) -> Self::T {
        Self::from_segment_with_config::<DefaultGivenVecConfig>(segment)
    }

    #[inline]
    fn vec_excess() -> usize {
        TCf::TAnyRc::overhead_bytes()
    }

    #[inline]
    fn from_given_vec(vec: Vec<u8>) -> Self::T {
        TCf::from_given_vec_with_config::<DefaultGivenVecConfig>(vec)
    }

    #[inline]
    fn from_given_vec_with_config<T: GivenVecConfig>(mut vec: Vec<u8>) -> Self::T {
        maybe_shrink::<T::TExcessShrink>(&mut vec, Self::vec_excess());
        // here we just check whether there's sufficient excess
        let excess = vec.capacity() - vec.len();
        let sufficient_excess = excess >= Self::vec_excess();

        if sufficient_excess {
            // perfect: sufficient excess for reference-counting
            TCf::TAnyRc::from_vec(vec)
        } else {
            // ok, here we have many choices, first try whether fits onto the stack.
            if let Some(stack) = StackBin::try_from(vec.as_slice()) {
                // perfect
                TCf::TFunctions::convert_to_un_sync(stack)
            } else {
                // now this step depends on the chosen optimization
                match T::optimization() {
                    GivenVecOptimization::Creation => {
                        // vector binary
                        TCf::TFunctions::convert_to_un_sync(VecBin::from_vec(
                            vec,
                            TCf::TFunctions::is_sync(),
                        ))
                    }
                    GivenVecOptimization::Operations => {
                        // we still use the rc-bin and hope that reserving additional capacity
                        // will not result in a completely new allocation & memory copy...
                        TCf::TAnyRc::from_vec(vec)
                    }
                }
            }
        }
    }
}

impl CommonFactory for New {
    type TAnyRc = RcBin;
    type TFunctions = FunctionsForNew;
}

pub struct FunctionsForNew {}

impl CommonFactoryFunctions for FunctionsForNew {
    type TSync = SBin;
    type TUnSync = Bin;

    #[inline]
    fn convert_to_un_sync(value: Self::TSync) -> Self::TUnSync {
        value.un_sync()
    }

    #[inline]
    fn is_sync() -> bool {
        false
    }
}

impl CommonFactory for SNew {
    type TAnyRc = ArcBin;
    type TFunctions = FunctionsForSNew;
}

pub struct FunctionsForSNew {}

impl CommonFactoryFunctions for FunctionsForSNew {
    type TSync = SBin;
    type TUnSync = SBin;

    #[inline]
    fn convert_to_un_sync(value: Self::TSync) -> Self::TUnSync {
        // does nothing
        value
    }

    fn is_sync() -> bool {
        true
    }
}
