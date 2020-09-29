use core::slice;

use crate::{Bin, FnTable, BinData, SyncBin, UnsafeBin, EmptyBin};

/// A binary from a static slice.
pub struct StaticBin;

impl StaticBin {
    pub const fn from(slice: &'static [u8]) -> SyncBin {
        let len = slice.len();
        if len == 0 {
            EmptyBin::new()
        } else {
            let ptr = slice.as_ptr();
            SyncBin(Bin::_const_new(BinData(ptr, len, 0), &FN_TABLE))
        }
    }
}

const FN_TABLE: FnTable = FnTable {
    drop,
    as_slice,
    is_empty,
    clone,
    into_vec,
    slice,
};

fn drop(_: &mut Bin) {
    // does nothing, static does not need to be dropped.
}

#[inline]
fn as_slice(bin: &Bin) -> &'static [u8] {
    unsafe {
        let data = bin._data();
        let ptr = data.0 as *const u8;
        let len = data.1;
        slice::from_raw_parts(ptr, len)
    }
}

fn is_empty(bin: &Bin) -> bool {
    let data = unsafe { bin._data() };
    let len = data.1;
    len == 0
}

fn clone(bin: &Bin) -> Bin {
    let data = unsafe { bin._data() };
    let ptr = data.0;
    let len = data.1;
    unsafe { Bin::_new(BinData(ptr, len, 0), &FN_TABLE) }
}

fn into_vec(bin: Bin) -> Vec<u8> {
    // the only option is to copy
    as_slice(&bin).to_vec()
}

fn slice(bin : &Bin, start: usize, end_excluded: usize) -> Option<Bin> {
    let self_slice = as_slice(bin);
    let new_slice = self_slice.get(start..end_excluded);
    if let Some(new_slice) = new_slice {
        Some(StaticBin::from(new_slice).un_sync())
    } else {
        None
    }
}