use crate::{BinConfig, BinData, SyncBin, UnsafeBin};
use std::marker::PhantomData;

#[repr(C)]
pub struct Bin {
    data: BinData,
    config: &'static BinConfig,
    // marker to make sure this is not send + sync
    _not_sync : PhantomData<*const u8>,
}

unsafe impl UnsafeBin for Bin {
    #[inline]
    fn _new(data: BinData, config: &'static BinConfig) -> Self {
        Self { data, config, _not_sync : PhantomData }
    }

    #[inline]
    fn _data(&self) -> &BinData {
        &self.data
    }

    #[inline]
    fn _config(&self) -> &'static BinConfig {
        self.config
    }

    #[inline]
    fn _into_sync(self) -> SyncBin {
        SyncBin(self)
    }
}

