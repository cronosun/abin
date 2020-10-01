use core::slice;
use std::mem;

use crate::{Bin, BinData, EmptyBin, FnTable, IntoUnSyncView, SyncBin, UnsafeBin};

/// A binary from a static slice.
pub struct StaticBin;

impl StaticBin {
    /// A static binary. Does never allocate memory.
    ///
    /// ```rust
    /// use abin::{StaticBin, AnyBin};
    ///
    /// let slice = "Hello, world!".as_bytes();
    /// let bin = StaticBin::from(slice);
    /// assert_eq!(bin.as_slice(), slice);
    /// ```
    pub fn from(slice: &'static [u8]) -> SyncBin {
        let len = slice.len();
        if len == 0 {
            EmptyBin::new()
        } else {
            let ptr = slice.as_ptr();
            let data = StaticBinData::new(ptr, len);
            SyncBin(unsafe { Bin::_new(data.to_bin_data(), &FN_TABLE) })
        }
    }
}

#[repr(C)]
struct StaticBinData {
    ptr: *const u8,
    len: usize,
    _unused: usize,
}

impl StaticBinData {
    #[inline]
    const fn new(ptr: *const u8, len: usize) -> Self {
        Self {
            ptr,
            len,
            _unused: 0,
        }
    }

    #[inline]
    unsafe fn from_bin(bin: &Bin) -> &Self {
        let bin_data = bin._data() as *const BinData;
        let self_data = mem::transmute::<*const BinData, *const Self>(bin_data);
        &*self_data
    }

    #[inline]
    unsafe fn to_bin_data(&self) -> BinData {
        mem::transmute_copy::<Self, BinData>(self)
    }
}

const FN_TABLE: FnTable = FnTable {
    // not required, no managed heap-memory
    drop: None,
    as_slice: Some(as_slice),
    is_empty: Some(is_empty),
    clone,
    into_vec,
    slice,
    // not required: sync only.
    convert_into_un_sync: None,
    // not required: sync only.
    convert_into_sync: None,
};

#[inline]
fn as_slice(bin: &Bin) -> &'static [u8] {
    let static_data = unsafe { StaticBinData::from_bin(bin) };
    let ptr = static_data.ptr;
    let len = static_data.len;
    unsafe { slice::from_raw_parts(ptr, len) }
}

fn is_empty(bin: &Bin) -> bool {
    let static_data = unsafe { StaticBinData::from_bin(bin) };
    let len = static_data.len;
    len == 0
}

fn clone(bin: &Bin) -> Bin {
    let static_data = unsafe { StaticBinData::from_bin(bin) };
    unsafe { Bin::_new(static_data.to_bin_data(), &FN_TABLE) }
}

fn into_vec(bin: Bin) -> Vec<u8> {
    // the only option is to copy
    as_slice(&bin).to_vec()
}

fn slice(bin: &Bin, start: usize, end_excluded: usize) -> Option<Bin> {
    let self_slice = as_slice(bin);
    let new_slice = self_slice.get(start..end_excluded);
    if let Some(new_slice) = new_slice {
        Some(StaticBin::from(new_slice).un_sync())
    } else {
        None
    }
}
