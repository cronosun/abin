use core::{mem, slice};
use core::sync::atomic;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::atomic::Ordering::{Relaxed, Release};

use crate::{AnyBin, Bin, FnTable, BinData, NoVecCapShrink, StackBin, SyncBin, UnsafeBin, VecCapShrink};

/// we use u32 (4 bytes) for reference counts. This should be more than enough for most use cases.
const RC_LEN_BYTES: usize = 4;
/// maximum padding required for the reference count.
const RC_MAX_PADDING: usize = RC_LEN_BYTES - 1;
const RC_OVERHEAD: usize = RC_LEN_BYTES + RC_MAX_PADDING;

/// A reference counted binary: depending on the `FnTable` it's send+sync or not.
///
/// Internal data layout:
///
///  * data.0: Pointer to the memory.
///  * data.1: length (NOT including padding + rc counter).
///  * data.2: capacity.
pub(crate) struct AnyRcImpl;

impl AnyRcImpl {
    #[inline]
    pub const fn overhead_bytes() -> usize {
        RC_OVERHEAD
    }

    #[inline]
    pub fn from_slice_not_sync(slice: &[u8]) -> Bin {
        if let Some(stack_bin) = StackBin::try_from(slice) {
            stack_bin.un_sync()
        } else {
            let vec = Self::vec_from_slice_with_capacity_for_rc(slice);
            // note: We never need a capacity shrink here (vector should already have the right capacity).
            Self::from::<NoVecCapShrink>(vec, &NS_FN_TABLE)
        }
    }

    #[inline]
    pub fn from_slice_sync(slice: &[u8]) -> SyncBin {
        if let Some(stack_bin) = StackBin::try_from(slice) {
            stack_bin
        } else {
            let vec = Self::vec_from_slice_with_capacity_for_rc(slice);
            // note: We never need a capacity shrink here (vector should already have the right capacity).
            unsafe { Self::from::<NoVecCapShrink>(vec, &SYNC_FN_TABLE)._into_sync() }
        }
    }

    #[inline]
    pub fn from_not_sync<T: VecCapShrink>(vec: Vec<u8>) -> Bin {
        if let Some(stack_bin) = StackBin::try_from(vec.as_slice()) {
            stack_bin.un_sync()
        } else {
            Self::from::<T>(vec, &NS_FN_TABLE)
        }
    }

    #[inline]
    pub fn from_sync<T: VecCapShrink>(vec: Vec<u8>) -> SyncBin {
        if let Some(stack_bin) = StackBin::try_from(vec.as_slice()) {
            stack_bin
        } else {
            unsafe { Self::from::<T>(vec, &SYNC_FN_TABLE)._into_sync() }
        }
    }

    #[inline]
    fn vec_from_slice_with_capacity_for_rc(slice: &[u8]) -> Vec<u8> {
        let slice_len = slice.len();
        let mut vec = Vec::with_capacity(slice_len + Self::overhead_bytes());
        vec.extend_from_slice(slice);
        vec
    }

    fn from<T: VecCapShrink>(mut vec: Vec<u8>, fn_table: &'static FnTable) -> Bin {
        let original_len = vec.len();
        let original_ptr = vec.as_ptr();

        // extend length to make sure we have enough space for the padding + reference counter.
        let padding = rc_padding(original_ptr, original_len);
        extend_with_padding_and_rc(&mut vec, padding);

        let len = vec.len();
        let capacity = vec.capacity();
        let is_shrink = capacity > T::min_capacity() && T::is_shrink(len, capacity);
        let capacity = if is_shrink {
            vec.shrink_to_fit();
            vec.capacity()
        } else {
            // still the same capacity
            capacity
        };

        let ptr = vec.as_ptr();
        // make sure vector memory is not freed
        mem::forget(vec);

        unsafe { Bin::_new(BinData(ptr, original_len, capacity), fn_table) }
    }
}

#[inline]
fn extend_with_padding_and_rc(vec: &mut Vec<u8>, padding: usize) {
    let zero_slice = &[0u8; RC_OVERHEAD];
    let zero_slice = &zero_slice[0..(padding + RC_LEN_BYTES)];
    vec.extend_from_slice(zero_slice);
}

/// This returns the number of padding bytes after the content to make sure the reference
/// count is aligned.
#[inline]
fn rc_padding(base_ptr: *const u8, len: usize) -> usize {
    let target = (base_ptr as usize) + len;
    // RC_LEN_BYTES bytes alignment
    let remainder = target % RC_LEN_BYTES;
    if remainder == 0 {
        // nice, already aligned
        0
    } else {
        let padding = RC_LEN_BYTES - remainder;
        assert!(padding <= RC_MAX_PADDING);
        padding
    }
}

/// The function table for the non-sync rc.
const NS_FN_TABLE: FnTable = FnTable {
    drop: ns_drop,
    as_slice,
    is_empty,
    clone: ns_clone,
    into_vec: ns_into_vec,
};

/// The function table for the sync rc.
const SYNC_FN_TABLE: FnTable = FnTable {
    drop: sync_drop,
    as_slice,
    is_empty,
    clone: sync_clone,
    into_vec: sync_into_vec,
};

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

#[inline]
unsafe fn ns_rc_ptr(ptr: *const u8, len: usize) -> *mut u32 {
    let padding = rc_padding(ptr, len);
    let rc_index = len + padding;
    let rc_mut = (ptr.add(rc_index)) as *mut u32;
    rc_mut
}

/// Decrements the ref count. Returns `false` if could not decrement (is already 0).
#[inline]
unsafe fn ns_decrement_rc(ptr: *const u8, len: usize) -> bool {
    let rc_ptr = ns_rc_ptr(ptr, len);
    let rc_value = *rc_ptr;
    if rc_value == 0 {
        false
    } else {
        *rc_ptr = rc_value - 1;
        true
    }
}

/// increments the ref count by one.
#[inline]
unsafe fn ns_increment_rc(ptr: *const u8, len: usize) {
    let rc_ptr = ns_rc_ptr(ptr, len);
    let rc_value = *rc_ptr;
    *rc_ptr = rc_value.checked_add(1).expect("Too many reference counts.");
}

fn ns_drop(bin: &mut Bin) {
    unsafe {
        let data = bin._data();
        let ptr = data.0;
        let len = data.1;

        if !ns_decrement_rc(ptr, len) {
            // could not decrement ref count. This means this was the last reference.
            // create a new vector, will immediately deallocate.
            let capacity = data.2;
            Vec::<u8>::from_raw_parts(ptr as *mut u8, 0, capacity);
        }
    }
}

/// cloning is just incrementing the reference count.
fn ns_clone(bin: &Bin) -> Bin {
    let data = unsafe { bin._data() };
    let ptr = data.0;
    let len = data.1;
    let capacity = data.2;
    unsafe { ns_increment_rc(ptr, len); }

    unsafe { Bin::_new(BinData(ptr, len, capacity), bin._fn_table()) }
}

fn ns_into_vec(bin: Bin) -> Vec<u8> {
    // this is sometimes almost a no-op (in case we only have one reference).
    let data = unsafe { bin._data() };
    let ptr = data.0;
    let len = data.1;
    let capacity = data.2;

    if !unsafe { ns_decrement_rc(ptr, len) } {
        // great, no cloning required
        mem::forget(bin);
        unsafe { Vec::<u8>::from_raw_parts(ptr as *mut u8, len, capacity) }
    } else {
        // we need to clone it (there's still other references)
        let slice = bin.as_slice();
        let vec = slice.to_vec();
        mem::forget(bin);
        vec
    }
}

#[inline]
unsafe fn sync_rc_ptr(ptr: *const u8, len: usize) -> *const AtomicU32 {
    let padding = rc_padding(ptr, len);
    let rc_index = len + padding;
    let rc_mut = (ptr.add(rc_index)) as *mut u32 as *const AtomicU32;
    rc_mut
}

/// Decrements the ref count. Returns `false` if there's no more references to this binary.
#[inline]
unsafe fn sync_decrement_rc(ptr: *const u8, len: usize) -> bool {
    let rc_ptr = sync_rc_ptr(ptr, len);
    // 'Release' seems to be ok according to the sources from "Arc" (rust std lib).
    // Note: After this call, the ref count will be u32::MAX (overflow) - but this should be
    // ok (since we do not need that value anymore).
    let previous_value = (*rc_ptr).fetch_sub(1, Release);
    if previous_value == 0 {
        // According to "Arc" (rust std lib) we also need this (don't know exactly why).
        atomic::fence(Ordering::Acquire);
        false
    } else {
        true
    }
}

/// increments the ref count by one.
#[inline]
unsafe fn sync_increment_rc(ptr: *const u8, len: usize) {
    let rc_ptr = sync_rc_ptr(ptr, len);
    // 'Relaxed' seems to be ok according to the sources from "Arc" (rust std lib).
    let previous_value = (*rc_ptr).fetch_add(1, Relaxed);
    if previous_value == std::u32::MAX {
        // we need to revert that
        (*rc_ptr).fetch_sub(1, Release);
        panic!("Too many reference counts.")
    }
}

fn sync_drop(bin: &mut Bin) {
    unsafe {
        let data = bin._data();
        let ptr = data.0;
        let len = data.1;

        if !sync_decrement_rc(ptr, len) {
            // last reference.
            // create a new vector, will immediately deallocate.
            let capacity = data.2;
            Vec::<u8>::from_raw_parts(ptr as *mut u8, 0, capacity);
        }
    }
}

/// cloning is just incrementing the reference count.
fn sync_clone(bin: &Bin) -> Bin {
    let data = unsafe { bin._data() };
    let ptr = data.0;
    let len = data.1;
    let capacity = data.2;
    unsafe { sync_increment_rc(ptr, len); }

    unsafe { Bin::_new(BinData(ptr, len, capacity), bin._fn_table()) }
}

fn sync_into_vec(bin: Bin) -> Vec<u8> {
    // this is sometimes almost a no-op (in case we only have one reference).
    let data = unsafe { bin._data() };
    let ptr = data.0;
    let len = data.1;
    let capacity = data.2;

    if !unsafe { sync_decrement_rc(ptr, len) } {
        // great, no cloning required
        mem::forget(bin);
        unsafe { Vec::<u8>::from_raw_parts(ptr as *mut u8, len, capacity) }
    } else {
        // we need to clone it (there's still other references)
        let slice = bin.as_slice();
        let vec = slice.to_vec();
        mem::forget(bin);
        vec
    }
}