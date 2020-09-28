use std::marker::PhantomData;

use crate::{BinConfig, BinData, SyncBin, UnsafeBin, AnyBin};
use std::ops::Deref;

#[repr(C)]
pub struct Bin {
    data: BinData,
    config: &'static BinConfig,
    // marker to make sure this is not send + sync
    _not_sync: PhantomData<*const u8>,
}

impl AnyBin for Bin {

}

impl Deref for Bin {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        (self.config.as_slice)(self)
    }
}

unsafe impl UnsafeBin for Bin {
    #[inline]
    unsafe fn _new(data: BinData, config: &'static BinConfig) -> Self {
        Self { data, config, _not_sync: PhantomData }
    }

    #[inline]
    unsafe fn _data(&self) -> &BinData {
        &self.data
    }

    #[inline]
    unsafe fn _data_mut(&mut self) -> &mut BinData {
        &mut self.data
    }

    #[inline]
    unsafe fn _config(&self) -> &'static BinConfig {
        self.config
    }

    #[inline]
    unsafe fn _into_sync(self) -> SyncBin {
        SyncBin(self)
    }
}

