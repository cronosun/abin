use core::{mem, slice};

use crate::{Bin, BinData, Factory, FnTable, IntoUnSyncView, maybe_shrink, NeverShrink, New, SBin, SNew, UnsafeBin};
use crate::{AnyRc, ArcBin, DefaultExcessShrink, ExcessShrink, StackBin};

/// A binary that is backed by a `Vec<u8>`. Note: It's not reference-counted:
/// If you clone it or slice it, it will be converted to a reference-counted version.
pub struct VecBin;

impl VecBin {
    /// Creates a new binary based on a vector.
    #[inline]
    pub fn from_vec(
        vec: Vec<u8>,
        synchronized: bool) -> SBin {
        let len = vec.len();
        let capacity = vec.capacity();

        let ptr = vec.as_ptr();
        // make sure vector memory is not freed
        mem::forget(vec);

        let fn_table = if synchronized {
            &FN_TABLE_SYNC
        } else {
            &FN_TABLE_NON_SYNC
        };
        let vec_data = VecData { ptr, len, capacity };

        unsafe { Bin::_new(vec_data.to_bin_data(), fn_table)._into_sync() }
    }
}

#[repr(C)]
struct VecData {
    ptr: *const u8,
    len: usize,
    capacity: usize,
}

impl VecData {
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

const FN_TABLE_SYNC: FnTable = FnTable {
    drop: Some(drop),
    as_slice: Some(as_slice),
    is_empty: Some(is_empty),
    clone: clone_sync,
    into_vec,
    slice: slice_sync,
    // convert to non-synchronized
    convert_into_un_sync: None,
    // not required, already sync
    convert_into_sync: None,
    // not supported
    try_re_integrate: None,
};

const FN_TABLE_NON_SYNC: FnTable = FnTable {
    drop: Some(drop),
    as_slice: Some(as_slice),
    is_empty: Some(is_empty),
    clone: clone_non_sync,
    into_vec,
    slice: slice_non_sync,
    // not required, already un-sync
    convert_into_un_sync: None,
    convert_into_sync: None,
    // not supported
    try_re_integrate: None,
};

fn drop(bin: &mut Bin) {
    let vec_data = unsafe { VecData::from_bin(bin) };
    let ptr = vec_data.ptr as *mut u8;
    let capacity = vec_data.capacity;
    // restore the original vector, will immediately drop
    unsafe {
        Vec::<u8>::from_raw_parts(ptr, 0, capacity);
    }
}

#[inline]
fn as_slice(bin: &Bin) -> &[u8] {
    let vec_data = unsafe { VecData::from_bin(bin) };
    let ptr = vec_data.ptr;
    let len = vec_data.len;
    unsafe { slice::from_raw_parts(ptr, len) }
}

#[inline]
fn len(bin: &Bin) -> usize {
    let vec_data = unsafe { VecData::from_bin(bin) };
    vec_data.len
}

fn is_empty(bin: &Bin) -> bool {
    let vec_data = unsafe { VecData::from_bin(bin) };
    let len = vec_data.len;
    len == 0
}

fn clone_sync(bin: &Bin) -> Bin {
    // this involves copying memory
    let slice = as_slice(bin);
    SNew::copy_from_slice(slice).un_sync()
}

fn clone_non_sync(bin: &Bin) -> Bin {
    // this involves copying memory
    let slice = as_slice(bin);
    New::copy_from_slice(slice)
}

fn into_vec(bin: Bin) -> Vec<u8> {
    // this is almost a no-op, since this is already backed by a vector.
    let vec_data = unsafe { VecData::from_bin(&bin) };
    let ptr = vec_data.ptr as *mut u8;
    let len = vec_data.len;
    let capacity = vec_data.capacity;
    // make sure drop is not called on `Bin` ... since we need the allocated buffer for the vec.
    mem::forget(bin);
    // restore the original vector
    unsafe { Vec::<u8>::from_raw_parts(ptr, len, capacity) }
}

fn slice_sync(bin: &Bin, start: usize, end_excluded: usize) -> Option<Bin> {
    if start == 0 && end_excluded == len(bin) {
        // this is myself
        Some(bin.clone())
    } else {
        let slice = as_slice(bin).get(start..end_excluded);
        if let Some(slice) = slice {
            Some(SNew::copy_from_slice(slice).un_sync())
        } else {
            None
        }
    }
}

fn slice_non_sync(bin: &Bin, start: usize, end_excluded: usize) -> Option<Bin> {
    if start == 0 && end_excluded == len(bin) {
        // this is myself
        Some(bin.clone())
    } else {
        let slice = as_slice(bin).get(start..end_excluded);
        if let Some(slice) = slice {
            Some(New::copy_from_slice(slice))
        } else {
            None
        }
    }
}
