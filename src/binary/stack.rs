use core::slice;

use crate::{Bin, BinConfig, BinData, SyncBin, UnsafeBin};

use crate::EmptyBin;

const BIN_DATA_LEN: usize = std::mem::size_of::<BinData>();
const I_BIN_DATA_LEN: isize = BIN_DATA_LEN as isize;

/// A binary that stores the content entirely on the stack.
pub struct StackBin;

impl StackBin {
    /// Does those steps:
    ///
    ///  * If it's empty, returns `EmptyBin::new()`.
    ///  * If the slice is small (less than the size of `BinData`) returns a stack binary.
    ///  * ...otherwise returns `None`.
    #[inline]
    pub fn try_from(slice: &[u8]) -> Option<SyncBin> {
        let len = slice.len();
        if len == 0 {
            // no problem, empty can always be stored on the stack
            Some(EmptyBin::new())
        } else if len < BIN_DATA_LEN {
            // yes, this works (one byte is required for the length information)
            let mut bin = unsafe { Bin::_new(BinData(0, 0, 0), &CONFIG) };
            let data_ptr = data_raw_mut(unsafe { bin._data_mut() });
            unsafe { core::ptr::copy(slice.as_ptr(), data_ptr, len); }
            let len = len as u8;
            unsafe { *data_ptr.offset(I_BIN_DATA_LEN) = len; }
            Some(unsafe { bin._into_sync() })
        } else {
            None
        }
    }
}

#[inline]
fn data_raw(data: &BinData) -> *const u8 {
    data as *const BinData as *const u8
}

#[inline]
fn data_raw_mut(data: &mut BinData) -> *mut u8 {
    data as *mut BinData as *mut u8
}

const CONFIG: BinConfig = BinConfig {
    drop,
    as_slice,
    is_empty,
    clone,
    into_vec,
};

fn drop(_: &mut Bin) {
    // does nothing (stack only).
}

#[inline]
fn as_slice(bin: &Bin) -> &[u8] {
    unsafe {
        let data = bin._data();
        let data = data_raw(data);
        let len: u8 = *data.offset(I_BIN_DATA_LEN);
        let len = len as usize;
        slice::from_raw_parts(data, len)
    }
}

fn is_empty(bin: &Bin) -> bool {
    let data = unsafe { bin._data() };
    let data = data_raw(data);
    let len: u8 = unsafe { *data.offset(I_BIN_DATA_LEN) };
    len == 0
}

fn clone(bin: &Bin) -> Bin {
    let data = unsafe { bin._data() };
    unsafe { Bin::_new(BinData(data.0, data.1, data.2), &CONFIG) }
}

fn into_vec(bin: Bin) -> Vec<u8> {
    as_slice(&bin).to_vec()
}