use std::marker::PhantomData;

use crate::{AnyBin, FnTable, BinData, SyncBin, UnsafeBin};

#[repr(C)]
pub struct Bin {
    data: BinData,
    fn_table: &'static FnTable,
    // marker to make sure this is not send + sync
    _not_sync: PhantomData<*const u8>,
}

impl AnyBin for Bin {
    #[inline]
    fn as_slice(&self) -> &[u8] {
        (self.fn_table.as_slice)(self)
    }

    #[inline]
    fn into_vec(self) -> Vec<u8> {
        (self.fn_table.into_vec)(self)
    }

    #[inline]
    fn len(&self) -> usize {
        (self.fn_table.as_slice)(self).len()
    }
}

impl Drop for Bin {
    fn drop(&mut self) {
        (self.fn_table.drop)(self)
    }
}

impl Clone for Bin {
    fn clone(&self) -> Self {
        (self.fn_table.clone)(self)
    }
}

impl Bin {
    /// This is required since we can't use `unsafe` in const fn but we need const new
    /// for the static bin.
    pub(crate) const fn _const_new(data: BinData, fn_table: &'static FnTable) -> Self {
        Self { data, fn_table, _not_sync: PhantomData }
    }
}

unsafe impl UnsafeBin for Bin {
    #[inline]
    unsafe fn _new(data: BinData, fn_table: &'static FnTable) -> Self {
        Self { data, fn_table, _not_sync: PhantomData }
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
    unsafe fn _fn_table(&self) -> &'static FnTable {
        self.fn_table
    }

    #[inline]
    unsafe fn _into_sync(self) -> SyncBin {
        SyncBin(self)
    }
}

