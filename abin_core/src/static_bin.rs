use core::slice;

use abin_interface::{Bin, BinConfig, BinData, SyncBin, UnsafeBin};

use crate::EmptyBin;

/// A binary from a static slice.
pub struct StaticBin;

impl StaticBin {
    pub fn new(slice: &'static [u8]) -> SyncBin {
        let len = slice.len();
        if len == 0 {
            EmptyBin::new()
        } else {
            let ptr = slice.as_ptr() as usize;
            unsafe { Bin::_new(BinData(ptr, len, 0), &CONFIG)._into_sync() }
        }
    }
}

const CONFIG: BinConfig = BinConfig {
    drop,
    as_slice,
    is_empty
};

fn drop(_: &mut Bin) {
    // does nothing, static does not need to be dropped.
}

fn as_slice(bin: &Bin) -> &[u8] {
    unsafe {
        let data = bin._data();
        let ptr = data.0 as *const u8;
        let len = data.1;
        slice::from_raw_parts(ptr, len)
    }
}

fn is_empty(bin : &Bin) -> bool {
    let data = bin._data();
    let len = data.1;
    len==0
}