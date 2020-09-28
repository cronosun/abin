use std::marker::PhantomData;
use std::ops::Deref;

use crate::{AnyBin, BinConfig, BinData, SyncBin, UnsafeBin};

#[repr(C)]
pub struct Bin {
    data: BinData,
    config: &'static BinConfig,
    // marker to make sure this is not send + sync
    _not_sync: PhantomData<*const u8>,
}

impl AnyBin for Bin {
    #[inline]
    fn as_slice(&self) -> &[u8] {
        (self.config.as_slice)(self)
    }

    #[inline]
    fn into_vec(self) -> Vec<u8> {
        (self.config.into_vec)(self)
    }
}

impl Deref for Bin {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        (self.config.as_slice)(self)
    }
}

impl Clone for Bin {
    fn clone(&self) -> Self {
        (self.config.clone)(self)
    }
}

impl Bin {
    /* TODO: For Static_Bin
    /// This is required since we can't use `unsafe` in const fn but we need const new
    /// for the static bin.
    pub(crate) const fn _const_new(data: BinData, config: &'static BinConfig) -> Self {
        Self { data, config, _not_sync: PhantomData }
    }*/
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

