use core::slice;

use abin_interface::{Bin, BinConfig, BinData, SyncBin, UnsafeBin};

use crate::EmptyBin;

/// A binary from a static slice.
pub struct StaticBin;

impl StaticBin {
    // TODO: Can we make this const fn?
    pub fn from(slice: &'static [u8]) -> SyncBin {
        let len = slice.len();
        if len == 0 {
            EmptyBin::new()
        } else {
            let ptr = slice.as_ptr() as usize;
            // TODO Bin::_const_new
            unsafe { Bin::_new(BinData(ptr, len, 0), &CONFIG)._into_sync() }
        }
    }
}

const CONFIG: BinConfig = BinConfig {
    drop,
    as_slice,
    is_empty,
    clone,
    into_vec,
};

fn drop(_: &mut Bin) {
    // does nothing, static does not need to be dropped.
}

#[inline]
fn as_slice(bin: &Bin) -> &[u8] {
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
    unsafe { Bin::_new(BinData(ptr, len, 0), &CONFIG) }
}

fn into_vec(bin: Bin) -> Vec<u8> {
    // the only option is to copy
    as_slice(&bin).to_vec()
}