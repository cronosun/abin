use crate::{AnyRc, AnyRcConfigForNonSync, AnyRcImpl, Bin};

/// A reference-counted binary. Note: The reference counter is not synchronized, so this
/// is not `Send + Sync` but there's less overhead. Cloning is cheap. See `AnyRc`.
pub struct RcBin;

impl AnyRc for RcBin {
    type T = Bin;

    #[inline]
    fn copy_from_slice(slice: &[u8]) -> Self::T {
        AnyRcImpl::<AnyRcConfigForNonSync>::copy_from_slice(slice)
    }

    #[inline]
    fn from_iter(iter: impl IntoIterator<Item = u8>) -> Self::T {
        AnyRcImpl::<AnyRcConfigForNonSync>::from_iter(iter)
    }

    #[inline]
    fn from_vec(vec: Vec<u8>) -> Self::T {
        AnyRcImpl::<AnyRcConfigForNonSync>::from_vec(vec)
    }

    #[inline]
    fn overhead_bytes() -> usize {
        AnyRcImpl::<AnyRcConfigForNonSync>::overhead_bytes()
    }
}
