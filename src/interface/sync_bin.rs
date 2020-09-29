use crate::{AnyBin, Bin, UnsafeBin};

pub struct SyncBin(pub(crate) Bin);

unsafe impl Sync for SyncBin {}

unsafe impl Send for SyncBin {}

impl Into<Bin> for SyncBin {
    #[inline]
    fn into(self) -> Bin {
        self.0
    }
}

impl SyncBin {
    #[inline]
    pub fn un_sync(self) -> Bin {
        self.0
    }
}

impl AnyBin for SyncBin {
    #[inline]
    fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    #[inline]
    fn into_vec(self) -> Vec<u8> {
        self.0.into_vec()
    }

    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl Clone for SyncBin {
    #[inline]
    fn clone(&self) -> Self {
        unsafe { self.0.clone()._into_sync() }
    }
}
