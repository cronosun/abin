use crate::{AnyRc, AnyRcConfigForNonSync, AnyRcImpl, Bin, VecCapShrink};

/// A reference-counted binary. Note: The reference counter is not synchronized, so this
/// is not sync + send but there's less overhead. Cloning is cheap.
pub struct RcBin;

impl AnyRc for RcBin {
    type T = Bin;

    #[inline]
    fn from_vec(vec: Vec<u8>) -> Self::T {
        AnyRcImpl::<AnyRcConfigForNonSync>::from_vec(vec)
    }

    #[inline]
    fn copy_from_slice(slice: &[u8]) -> Self::T {
        AnyRcImpl::<AnyRcConfigForNonSync>::copy_from_slice(slice)
    }

    #[inline]
    fn overhead_bytes() -> usize {
        AnyRcImpl::<AnyRcConfigForNonSync>::overhead_bytes()
    }

    #[inline]
    fn from_with_cap_shrink<T: VecCapShrink>(vec: Vec<u8>) -> Self::T {
        AnyRcImpl::<AnyRcConfigForNonSync>::from_with_cap_shrink::<T>(vec)
    }
}
