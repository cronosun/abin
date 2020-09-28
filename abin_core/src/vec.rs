use core::{mem, slice};

use abin_interface::{Bin, BinConfig, BinData, SyncBin, UnsafeBin};

use crate::{DefaultVecCapShrink, VecCapShrink, EmptyBin};

/// A binary that is backed by a `Vec<u8>`. Note: It's not reference-counted: So cloning
/// this binary will copy the content.
pub struct VecBin;

impl VecBin {
    #[inline]
    pub fn new(vec: Vec<u8>) -> SyncBin {
        Self::new_with_cap_shrink::<DefaultVecCapShrink>(vec)
    }

    pub fn new_with_cap_shrink<T: VecCapShrink>(mut vec: Vec<u8>) -> SyncBin {
        if vec.is_empty() {
            EmptyBin::new()
        } else {
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

            unsafe { Bin::_new(BinData(ptr, len, capacity), &CONFIG)._into_sync() }
        }
    }
}


const CONFIG: BinConfig = BinConfig {
    drop,
    as_slice,
    is_empty,
};

fn drop(bin: &mut Bin) {
    unsafe {
        let data = bin._data();
        let ptr = data.0 as *mut u8;
        let len = data.1;
        let capacity = data.1;
        // restore the original vector, will immediately drop
        Vec::<u8>::from_raw_parts(ptr as *mut u8, len, capacity);
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


