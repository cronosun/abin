use core::slice;

use crate::{AnyBin, Bin, BinData, FnTable, SyncBin, UnsafeBin};
use crate::{EmptyBin, IntoUnSyncView};

/// the number of bytes we can store + 1 (since one byte is required for the length information).
const BIN_DATA_LEN: usize = std::mem::size_of::<BinData>();
/// the offset where to store the length information.
const LENGTH_OFFSET: usize = BIN_DATA_LEN - 1;
/// the maximum number of bytes we can store (one byte is required for the length information)
const STACK_MAX_LEN: usize = BIN_DATA_LEN - 1;

/// A binary that stores the content entirely on the stack.
pub struct StackBin;

impl StackBin {
    /// Does those steps:
    ///
    ///  * If it's empty, returns `EmptyBin::new()`.
    ///  * If the slice is small (less than the size of `BinData`) returns a stack binary.
    ///  * ...otherwise returns `None`.
    ///
    /// ```rust
    /// use abin::{StackBin, SyncBin, AnyBin};
    ///
    /// let maximum_for_stack : Vec<u8> = (0..StackBin::max_len())
    ///   .map(|item| (item*2) as u8).collect();
    /// let bin : SyncBin = StackBin::try_from(maximum_for_stack.as_slice())
    ///   .expect("StackBin::max_len() must fit onto the stack.");
    /// assert_eq!(bin.as_slice(), maximum_for_stack.as_slice());
    ///
    /// let too_long_for_stack : Vec<u8> = (0..StackBin::max_len() + 1)
    ///   .map(|item| (item*2) as u8).collect();
    /// assert_eq!(None, StackBin::try_from(too_long_for_stack.as_slice()));
    /// ```
    #[inline]
    pub fn try_from(slice: &[u8]) -> Option<SyncBin> {
        let len = slice.len();
        if len == 0 {
            // no problem, empty can always be stored on the stack
            Some(EmptyBin::new())
        } else if len < BIN_DATA_LEN {
            // yes, this works (one byte is required for the length information)
            let mut bin = unsafe { Bin::_new(BinData::empty(), &FN_TABLE) };
            let data_ptr = data_raw_mut(unsafe { bin._data_mut() });
            unsafe {
                core::ptr::copy(slice.as_ptr(), data_ptr, len);
            }
            let len = len as u8;
            unsafe {
                *data_ptr.add(LENGTH_OFFSET) = len;
            }
            Some(unsafe { bin._into_sync() })
        } else {
            None
        }
    }

    /// The maximum number of bytes that can be stored on the stack.
    ///
    /// Note: This is platform-dependant: It's less on 32-bit machines, more on 64-bit machines.
    pub const fn max_len() -> usize {
        STACK_MAX_LEN
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

const FN_TABLE: FnTable = FnTable {
    // not required: Stack only.
    drop: None,
    as_slice: Some(as_slice),
    is_empty: Some(is_empty),
    clone,
    into_vec,
    slice,
    // not required: there's only a sync version.
    convert_into_un_sync: None,
    // not required: there's only a sync version.
    convert_into_sync: None,
};

#[inline]
fn as_slice(bin: &Bin) -> &[u8] {
    unsafe {
        let data = bin._data();
        let data = data_raw(data);
        let len: u8 = *data.add(LENGTH_OFFSET);
        let len = len as usize;
        slice::from_raw_parts(data, len)
    }
}

fn is_empty(bin: &Bin) -> bool {
    let data = unsafe { bin._data() };
    let data = data_raw(data);
    let len: u8 = unsafe { *data.add(LENGTH_OFFSET) };
    len == 0
}

fn clone(bin: &Bin) -> Bin {
    let data = unsafe { bin._data() };
    unsafe { Bin::_new(data.clone(), &FN_TABLE) }
}

fn into_vec(bin: Bin) -> Vec<u8> {
    as_slice(&bin).to_vec()
}

fn slice(bin: &Bin, start: usize, end_excluded: usize) -> Option<Bin> {
    let new_slice = bin.as_slice().get(start..end_excluded);
    if let Some(new_slice) = new_slice {
        Some(
            StackBin::try_from(new_slice)
                .expect(
                    "There's an implementation error: Was \
        unable to slice a stack bin and re-create a stack bin from that slice (this MUST never \
        fail, since the slice is never longer than the original).",
                )
                .un_sync(),
        )
    } else {
        None
    }
}
