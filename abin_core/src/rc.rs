use abin_interface::Bin;

use crate::{AnyRc, DefaultVecCapShrink, VecCapShrink};

/// A reference-counted binary. Note: The reference counter is not synchronized, so this
/// is not sync + send but there's less overhead. Cloning is cheap.
pub struct RcBin;

impl RcBin {
    #[inline]
    pub fn from(vec: Vec<u8>) -> Bin {
        AnyRc::from_not_sync::<DefaultVecCapShrink>(vec)
    }

    #[inline]
    pub fn copy_from_slice(slice : &[u8]) -> Bin {
        AnyRc::from_slice_not_sync(slice)
    }

    #[inline]
    pub fn from_with_cap_shrink<T: VecCapShrink>(vec: Vec<u8>) -> Bin {
        AnyRc::from_not_sync::<T>(vec)
    }
}