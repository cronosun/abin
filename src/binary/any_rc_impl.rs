use core::{mem, slice};

use crate::{NoVecCapShrink, StackBin, VecCapShrink, Bin, SyncBin, UnsafeBin, BinConfig, BinData, AnyBin};

/// we use u32 (4 bytes) for reference counts. This should be more than enough for most use cases.
const RC_LEN_BYTES: usize = 4;
/// maximum padding required for the reference count.
const RC_MAX_PADDING: usize = RC_LEN_BYTES - 1;
const RC_OVERHEAD: usize = RC_LEN_BYTES + RC_MAX_PADDING;

/// A reference counted binary: depending on the configuration it's send+sync or not.
///
/// Internal data layout:
///
///  * data.0: Pointer to the memory.
///  * data.1: length NOT including padding + rc counter.
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
            Self::from::<NoVecCapShrink>(vec, &NS_CONFIG)
        }
    }

    #[inline]
    pub fn from_slice_sync(slice: &[u8]) -> SyncBin {
        if let Some(stack_bin) = StackBin::try_from(slice) {
            stack_bin
        } else {
            let vec = Self::vec_from_slice_with_capacity_for_rc(slice);
            // note: We never need a capacity shrink here (vector should already have the right capacity).
            unsafe { Self::from::<NoVecCapShrink>(vec, &SYNC_CONFIG)._into_sync() }
        }
    }

    #[inline]
    pub fn from_not_sync<T: VecCapShrink>(vec: Vec<u8>) -> Bin {
        if let Some(stack_bin) = StackBin::try_from(vec.as_slice()) {
            stack_bin.un_sync()
        } else {
            Self::from::<T>(vec, &NS_CONFIG)
        }
    }

    #[inline]
    pub fn from_sync<T: VecCapShrink>(vec: Vec<u8>) -> SyncBin {
        if let Some(stack_bin) = StackBin::try_from(vec.as_slice()) {
            stack_bin
        } else {
            unsafe { Self::from::<T>(vec, &SYNC_CONFIG)._into_sync() }
        }
    }

    #[inline]
    fn vec_from_slice_with_capacity_for_rc(slice: &[u8]) -> Vec<u8> {
        let slice_len = slice.len();
        let mut vec = Vec::with_capacity(slice_len + Self::overhead_bytes());
        vec.extend_from_slice(slice);
        vec
    }

    fn from<T: VecCapShrink>(mut vec: Vec<u8>, config: &'static BinConfig) -> Bin {
        let original_len = vec.len();
        let original_ptr = vec.as_ptr();

        // extend length to make sure we have enough space for the padding + reference counter.
        let padding = rc_padding(original_ptr, original_len);
        extend_with_padding_and_rc(&mut vec, padding);

        let len = vec.len();
        let capacity = vec.len();
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

        unsafe { Bin::_new(BinData(ptr, original_len, capacity), config) }
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

const NS_CONFIG: BinConfig = BinConfig {
    drop: ns_drop,
    as_slice,
    is_empty,
    clone: ns_clone,
    into_vec: ns_into_vec,
};

const SYNC_CONFIG: BinConfig = BinConfig {
    drop: ns_drop, // TODO
    as_slice,
    is_empty,
    clone: ns_clone, // TODO
    into_vec: ns_into_vec, // TODO
};

/// Decrements the ref count. Returns `false` if could not decrement (is already 0).
#[inline]
unsafe fn ns_decrement_rc(ptr: *const u8, len: usize) -> bool {
    let padding = rc_padding(ptr, len);
    let rc_index = len + padding;
    let rc_mut = (ptr.add(rc_index)) as *mut u32;
    let rc_value = core::ptr::read(rc_mut);
    if rc_value == 0 {
        false
    } else {
        // decrement reference counter.
        core::ptr::write(rc_mut, rc_value - 1);
        true
    }
}

/// increments the ref count by one.
#[inline]
unsafe fn ns_increment_rc(ptr: *const u8, len: usize) {
    let padding = rc_padding(ptr, len);
    let rc_index = len + padding;
    let rc_mut = (ptr.add(rc_index)) as *mut u32;
    let rc_value = core::ptr::read(rc_mut);
    core::ptr::write(rc_mut, rc_value + 1);
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

/// cloning is just incrementing the reference count.
fn ns_clone(bin: &Bin) -> Bin {
    let data = unsafe { bin._data() };
    let ptr = data.0;
    let len = data.1;
    let capacity = data.2;
    unsafe { ns_increment_rc(ptr, len); }

    unsafe { Bin::_new(BinData(ptr, len, capacity), bin._config()) }
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