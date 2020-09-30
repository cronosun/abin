use core::{mem, slice};

use crate::{is_shrink, Bin, BinData, FnTable, NoVecCapShrink, SyncBin, UnsafeBin};
use crate::{AnyRc, ArcBin, DefaultVecCapShrink, StackBin, VecCapShrink};

/// If this threshold is reached, clone and slice won't return a vec again, they will return
/// an arc-bin instead.
const TO_ARC_THRESHOLD_BYTES: usize = 2048;

/// A binary that is backed by a `Vec<u8>`. Note: It's not reference-counted: So cloning
/// this binary will be expensive. Use this if you're quite sure that the binary won't be cloned.
pub struct VecBin;

impl VecBin {
    /// See `Self::from_with_cap_shrink`.
    #[inline]
    pub fn from(vec: Vec<u8>, allow_optimization: bool) -> SyncBin {
        Self::from_with_cap_shrink::<DefaultVecCapShrink>(vec, allow_optimization)
    }

    /// Creates a new binary based on a vector.
    ///
    /// `allow_optimization`: If this is true, the implementation is allowed to perform
    /// optimizations: If the given vector is small, it's allowed to choose a stack-binary
    /// instead - if the vector is large (and has enough capacity for the reference counter),
    /// it's allowed to use a `ArcBin` instead. This behaviour is also applied for `clone` and
    /// `slice` (recursively). You most likely want this to be `true`; but if you just use `Bin`
    /// as a container for `Vec<u8>` and then just unwrap it using `Bin::into_vec()` `false` might
    /// be a good choice too.
    #[inline]
    pub fn from_with_cap_shrink<T: VecCapShrink>(
        vec: Vec<u8>,
        allow_optimization: bool,
    ) -> SyncBin {
        if !allow_optimization {
            Self::from_non_optimized::<T>(vec, allow_optimization)
        } else {
            if let Some(stack_bin) = StackBin::try_from(vec.as_slice()) {
                stack_bin
            } else {
                let len = vec.len();
                if len > TO_ARC_THRESHOLD_BYTES && vec.capacity() - len >= ArcBin::overhead_bytes()
                {
                    ArcBin::from_with_cap_shrink::<T>(vec)
                } else {
                    Self::from_non_optimized::<T>(vec, allow_optimization)
                }
            }
        }
    }

    /// Creates a vector from given slice.
    ///
    /// See `from_with_cap_shrink` for `allow_optimization`.
    #[inline]
    pub fn copy_from_slice(slice: &[u8], allow_optimization: bool) -> SyncBin {
        if !allow_optimization {
            let vec = Vec::from(slice);
            Self::from_with_cap_shrink::<NoVecCapShrink>(vec, false)
        } else {
            if let Some(stack_bin) = StackBin::try_from(slice) {
                stack_bin
            } else if slice.len() > TO_ARC_THRESHOLD_BYTES {
                ArcBin::copy_from_slice(slice)
            } else {
                let vec = Vec::from(slice);
                Self::from_with_cap_shrink::<NoVecCapShrink>(vec, true)
            }
        }
    }

    #[inline]
    fn from_non_optimized<T: VecCapShrink>(mut vec: Vec<u8>, allow_optimization: bool) -> SyncBin {
        let len = vec.len();
        let capacity = vec.capacity();
        let shrink = is_shrink::<T>(len, capacity);
        let (len, capacity) = if shrink {
            vec.shrink_to_fit();
            (vec.len(), vec.capacity())
        } else {
            (len, capacity)
        };

        let ptr = vec.as_ptr();
        // make sure vector memory is not freed
        mem::forget(vec);

        let fn_table = if allow_optimization {
            &FN_TABLE_OPT
        } else {
            &FN_TABLE_NO_OPT
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

const FN_TABLE_NO_OPT: FnTable = FnTable {
    drop,
    as_slice,
    is_empty,
    clone: clone_no_opt,
    into_vec,
    slice: slice_no_opt,
};

const FN_TABLE_OPT: FnTable = FnTable {
    drop,
    as_slice,
    is_empty,
    clone: clone_opt,
    into_vec,
    slice: slice_opt,
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

fn is_empty(bin: &Bin) -> bool {
    let vec_data = unsafe { VecData::from_bin(bin) };
    let len = vec_data.len;
    len == 0
}

fn clone_no_opt(bin: &Bin) -> Bin {
    // this involves copying memory
    let slice = as_slice(bin);
    VecBin::copy_from_slice(slice, false).un_sync()
}

fn clone_opt(bin: &Bin) -> Bin {
    // this involves copying memory
    let slice = as_slice(bin);
    VecBin::copy_from_slice(slice, true).un_sync()
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

fn slice_no_opt(bin: &Bin, start: usize, end_excluded: usize) -> Option<Bin> {
    let slice = as_slice(bin).get(start..end_excluded);
    if let Some(slice) = slice {
        Some(VecBin::copy_from_slice(slice, false).un_sync())
    } else {
        None
    }
}

fn slice_opt(bin: &Bin, start: usize, end_excluded: usize) -> Option<Bin> {
    let slice = as_slice(bin).get(start..end_excluded);
    if let Some(slice) = slice {
        Some(VecBin::copy_from_slice(slice, true).un_sync())
    } else {
        None
    }
}
