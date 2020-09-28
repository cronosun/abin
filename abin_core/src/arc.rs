use abin_interface::{SyncBin};

use crate::{DefaultVecCapShrink, VecCapShrink, AnyRc};

/// A reference-counted binary. Note: The reference counter is synchronized, so this
/// is sync + send. Cloning is cheap.
pub struct ArcBin;

impl ArcBin {
    #[inline]
    pub fn from(vec: Vec<u8>) -> SyncBin {
        AnyRc::from_sync::<DefaultVecCapShrink>(vec)
    }

    #[inline]
    pub fn copy_from_slice(slice: &[u8]) -> SyncBin {
        AnyRc::from_slice_sync(slice)
    }

    #[inline]
    pub fn from_with_cap_shrink<T: VecCapShrink>(vec: Vec<u8>) -> SyncBin {
        AnyRc::from_sync::<T>(vec)
    }
}