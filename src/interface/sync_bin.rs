use crate::Bin;

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