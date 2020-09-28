use core::{mem, slice};

use abin_interface::{Bin, BinConfig, BinData, UnsafeBin};

use crate::{DefaultVecCapShrink, EmptyBin, StackBin, VecCapShrink};

/// we use u32 (4 bytes) for reference counts. This should be more than enough for most use cases.
const RC_LEN_BYTES: usize = 4;

/// A reference counted binary: depending on the configuration it's send+sync or not.
pub(crate) struct AnyRc;

impl AnyRc {
    #[inline]
    pub fn from_not_sync<T: VecCapShrink>(vec: Vec<u8>) -> Bin {
        if let Some(stack_bin) = StackBin::try_from(vec.as_slice()) {
            stack_bin.un_sync()
        } else {
            Self::from::<T>(vec, &NS_CONFIG)
        }
    }

    fn from<T: VecCapShrink>(mut vec: Vec<u8>, config: &'static BinConfig) -> Bin {
        let original_len = vec.len();
        let original_ptr = vec.as_ptr() as usize;

        // extend length to make sure we have enough space for the padding + reference counter.
        let padding = rc_padding(original_ptr, original_len);
        extend_with_padding_and_rc(&mut vec, padding);

        let len = vec.len();
        let capacity = vec.len();
        let is_shrink = (capacity > T::min_capacity() && T::is_shrink(len, capacity));
        let (len, capacity) = if is_shrink {
            vec.shrink_to_fit();
            (vec.len(), vec.capacity())
        } else {
            (len, capacity)
        };

        let ptr = vec.as_ptr() as usize;
        // make sure vector memory is not freed
        mem::forget(vec);

        unsafe { Bin::_new(BinData(ptr, len, capacity), config) }
    }
}

#[inline]
fn extend_with_padding_and_rc(vec: &mut Vec<u8>, padding: usize) {
    let zero_slice = &[0u8; RC_LEN_BYTES * 2];
    let zero_slice = &zero_slice[0..(padding + RC_LEN_BYTES)];
    vec.extend_from_slice(zero_slice);
}

/// This returns the number of padding bytes after the content to make sure the reference
/// count is aligned.
#[inline]
fn rc_padding(base_ptr: usize, len: usize) -> usize {
    let target = base_ptr + len;
    // RC_LEN_BYTES bytes alignment
    let remainder = target % RC_LEN_BYTES;
    if remainder == 0 {
        // nice, already aligned
        0
    } else {
        RC_LEN_BYTES - remainder
    }
}

const NS_CONFIG: BinConfig = BinConfig {
    drop: ns_drop,
    as_slice,
    is_empty,
};

/// Decrements the ref counter. Returns `false` if could not decrement (is already 0).
#[inline]
unsafe fn ns_decrement_rc(ptr: usize, len: usize) -> bool {
    let padding = rc_padding(ptr, len);
    let rc_index = len + padding;
    let rc_mut = (ptr + rc_index) as *mut u32;
    let rc_value = core::ptr::read(rc_mut);
    if rc_value == 0 {
        false
    } else {
        // decrement reference counter.
        core::ptr::write(rc_mut, rc_value - 1);
        true
    }
}

fn ns_drop(bin: &mut Bin) {
    unsafe {
        let data = bin._data();
        let ptr = data.0;
        let len = data.1;

        if !ns_decrement_rc(ptr, len) {
            // could not decrement ref count. This means this was the last reference.
            // create a new vector, will immediately deallocate.
            let capacity = data.1;
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
    let data = bin._data();
    let len = data.1;
    len == 0
}
