use core::{mem, slice};

use abin_interface::{AnyBin, Bin, BinConfig, BinData, SyncBin, UnsafeBin};

use crate::{ArcBin, DefaultVecCapShrink, StackBin, VecCapShrink};

const CLONE_TO_ARC_THRESHOLD_BYTES: usize = 2048;

/// A binary that is backed by a `Vec<u8>`. Note: It's not reference-counted: So cloning
/// this binary will be expensive. Use this if you're quite sure that the binary won't be cloned.
pub struct VecBin;

impl VecBin {
    #[inline]
    pub fn from(vec: Vec<u8>) -> SyncBin {
        Self::from_with_cap_shrink::<DefaultVecCapShrink>(vec)
    }

    pub fn from_with_cap_shrink<T: VecCapShrink>(mut vec: Vec<u8>) -> SyncBin {
        if let Some(stack_bin) = StackBin::try_from(vec.as_slice()) {
            stack_bin
        } else {
            let len = vec.len();
            let capacity = vec.len();
            let is_shrink = capacity > T::min_capacity() && T::is_shrink(len, capacity);
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
    clone,
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
    let data = unsafe { bin._data() };
    let len = data.1;
    len == 0
}

fn clone(bin: &Bin) -> Bin {
    // this involves copying memory
    let slice = bin.as_slice();
    if slice.len() < CLONE_TO_ARC_THRESHOLD_BYTES {
        // again, a vec
        let vec = bin.as_slice().to_vec();
        VecBin::from(vec).un_sync()
    } else {
        // it's too large, just create a arc
        ArcBin::copy_from_slice(slice).un_sync()
    }
}


