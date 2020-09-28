use crate::SyncBin;

use crate::{AnyRc, AnyRcImpl, DefaultVecCapShrink, VecCapShrink};

/// A reference-counted binary. Note: The reference counter is synchronized, so this
/// is sync + send. Cloning is cheap.
pub struct ArcBin;

impl AnyRc for ArcBin {
    type T = SyncBin;

    #[inline]
    fn from(vec: Vec<u8>) -> Self::T {
        AnyRcImpl::from_sync::<DefaultVecCapShrink>(vec)
    }

    #[inline]
    fn copy_from_slice(slice: &[u8]) -> Self::T {
        AnyRcImpl::from_slice_sync(slice)
    }

    #[inline]
    fn overhead_bytes() -> usize {
        AnyRcImpl::overhead_bytes()
    }

    #[inline]
    fn from_with_cap_shrink<T: VecCapShrink>(vec: Vec<u8>) -> Self::T {
        AnyRcImpl::from_sync::<T>(vec)
    }
}
