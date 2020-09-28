use abin_interface::Bin;

use crate::{AnyRc, AnyRcImpl, DefaultVecCapShrink, VecCapShrink};

/// A reference-counted binary. Note: The reference counter is not synchronized, so this
/// is not sync + send but there's less overhead. Cloning is cheap.
pub struct RcBin;

impl AnyRc for RcBin {
    type T = Bin;

    #[inline]
    fn from(vec: Vec<u8>) -> Self::T {
        AnyRcImpl::from_not_sync::<DefaultVecCapShrink>(vec)
    }

    #[inline]
    fn copy_from_slice(slice: &[u8]) -> Self::T {
        AnyRcImpl::from_slice_not_sync(slice)
    }

    #[inline]
    fn overhead_bytes() -> usize {
        AnyRcImpl::overhead_bytes()
    }

    #[inline]
    fn from_with_cap_shrink<T: VecCapShrink>(vec: Vec<u8>) -> Self::T {
        AnyRcImpl::from_not_sync::<T>(vec)
    }
}
