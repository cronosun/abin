use crate::{AnyRc, VecCapShrink};
use crate::{AnyRcConfigForSync, AnyRcImpl, SyncBin, UnsafeBin};

/// A reference-counted binary. Note: The reference counter is synchronized, so this
/// is `Send + Sync`. Cloning is cheap. See [AnyRc](trait.AnyRc.html) for details.
pub struct ArcBin;

impl AnyRc for ArcBin {
    type T = SyncBin;

    #[inline]
    fn copy_from_slice(slice: &[u8]) -> Self::T {
        unsafe { AnyRcImpl::<AnyRcConfigForSync>::copy_from_slice(slice)._into_sync() }
    }

    #[inline]
    fn from_iter(iter: impl IntoIterator<Item = u8>) -> Self::T {
        unsafe { AnyRcImpl::<AnyRcConfigForSync>::from_iter(iter)._into_sync() }
    }

    #[inline]
    fn from_vec(vec: Vec<u8>) -> Self::T {
        unsafe { AnyRcImpl::<AnyRcConfigForSync>::from_vec(vec)._into_sync() }
    }

    #[inline]
    fn overhead_bytes() -> usize {
        AnyRcImpl::<AnyRcConfigForSync>::overhead_bytes()
    }

    #[inline]
    fn from_with_cap_shrink<T: VecCapShrink>(vec: Vec<u8>) -> Self::T {
        unsafe { AnyRcImpl::<AnyRcConfigForSync>::from_with_cap_shrink::<T>(vec)._into_sync() }
    }
}
