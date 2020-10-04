use crate::{
    maybe_shrink, AnyBin, AnyRc, ArcBin, Bin, BinSegment, DefaultGivenVecConfig, EmptyBin, BinFactory,
    GivenVecConfig, GivenVecOptimization, IntoUnSyncView, NewBin, NewSBin, RcBin, SBin,
    SegmentIterator, StackBin, StackBinBuilder, StaticBin, VecBin,
};

pub trait CommonFactory {
    type TAnyRc: AnyRc;
    type TFunctions: CommonFactoryFunctions<TSync = SBin, TUnSync = <Self::TAnyRc as AnyRc>::T>;
}

pub trait CommonFactoryFunctions {
    type TSync;
    type TUnSync;
    fn convert_to_un_sync(value: Self::TSync) -> Self::TUnSync;
    fn is_sync() -> bool;
}

/// the vec capacity if the iter length is unknown.
const VEC_CAPACITY_IF_UNKNOWN_ITER_LEN: usize = 128;
/// If iterator returns some very high value, make sure to limit the capacity.
const VEC_CAPACITY_FROM_ITER_SAFE_MAX: usize = 1024 * 1024;

impl<TCf> BinFactory for TCf
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
    fn from_iter_with_config<T: GivenVecConfig, TIterator>(iter: TIterator) -> Self::T
    where
        TIterator: IntoIterator<Item = u8>,
    {
        let iter = iter.into_iter();
        match iter.size_hint() {
            (_, Some(max)) if max <= StackBin::max_len() => {
                // maybe will fit into stack
                let mut stack_bin_builder = StackBinBuilder::new(Self::vec_excess());
                for item in iter {
                    stack_bin_builder.extend_from_slice(&[item]);
                }
                match stack_bin_builder.build() {
                    Ok(stack) => TCf::TFunctions::convert_to_un_sync(stack),
                    Err(vec) => {
                        // returned wrong length
                        Self::from_given_vec_with_config::<T>(vec)
                    }
                }
            }
            (_, Some(max)) => {
                // does know length but it's too long for the stack
                let limited_max = core::cmp::min(max, VEC_CAPACITY_FROM_ITER_SAFE_MAX);
                let mut vec = Vec::with_capacity(limited_max + Self::vec_excess());
                vec.extend(iter);
                Self::from_given_vec_with_config::<T>(vec)
            }
            _ => {
                // seems to be long or does not know length (use a normal vec).
                let mut vec =
                    Vec::with_capacity(Self::vec_excess() + VEC_CAPACITY_IF_UNKNOWN_ITER_LEN);
                vec.extend(iter);
                Self::from_given_vec_with_config::<T>(vec)
            }
        }
    }

    #[inline]
    fn from_iter(iter: impl IntoIterator<Item = u8>) -> Self::T {
        Self::from_iter_with_config::<DefaultGivenVecConfig, _>(iter)
    }

    #[inline]
    fn from_segments_with_config<'a, T: GivenVecConfig, TIterator>(iter: TIterator) -> Self::T
    where
        TIterator: SegmentIterator<BinSegment<'a, Self::T>>,
    {
        if iter.is_empty() {
            TCf::TFunctions::convert_to_un_sync(EmptyBin::new())
        } else {
            match iter.single() {
                Ok(single) => {
                    // nice, one single item
                    Self::from_segment_with_config::<T, _>(single)
                }
                Err(iter) => {
                    // no, we have multiple segments ... would be nice if length is known
                    match iter.exact_number_of_bytes() {
                        (Some(number_of_bytes)) if number_of_bytes > StackBin::max_len() => {
                            // to long for stack ... collect into a vec (at least we know the exact capacity).
                            let mut vec: Vec<u8> =
                                Vec::with_capacity(number_of_bytes + Self::vec_excess());
                            for item in iter {
                                vec.extend_from_slice(item.as_slice())
                            }
                            Self::from_given_vec_with_config::<T>(vec)
                        }
                        _ => {
                            // we don't know the length or it's small enough for stack (both cases
                            // share the same code).
                            let mut stack_builder = StackBinBuilder::new(Self::vec_excess());
                            for item in iter {
                                stack_builder.extend_from_slice(item.as_slice())
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
    fn from_segments<'a>(iter: impl SegmentIterator<BinSegment<'a, Self::T>>) -> Self::T {
        Self::from_segments_with_config::<'a, DefaultGivenVecConfig, _>(iter)
    }

    #[inline]
    fn from_segment_with_config<'a, T: GivenVecConfig, TSegment>(segment: TSegment) -> Self::T
    where
        TSegment: Into<BinSegment<'a, Self::T>>,
    {
        let segment = segment.into();
        match segment {
            BinSegment::Slice(slice) => Self::copy_from_slice(slice),
            BinSegment::Static(slice) => Self::from_static(slice),
            BinSegment::Bin(bin) => bin,
            BinSegment::GivenVec(vec) => Self::from_given_vec_with_config::<T>(vec),
            BinSegment::Empty => Self::empty(),
            BinSegment::Bytes128(bytes) => Self::copy_from_slice(bytes.as_slice()),
        }
    }

    #[inline]
    fn from_segment<'a>(segment: impl Into<BinSegment<'a, Self::T>>) -> Self::T {
        Self::from_segment_with_config::<'a, DefaultGivenVecConfig, _>(segment)
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

impl CommonFactory for NewBin {
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

impl CommonFactory for NewSBin {
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
