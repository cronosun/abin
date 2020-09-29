use core::ptr;

use crate::RcCounter;

/// The metadata stored inside the vector (in-line).
#[repr(C)]
pub struct RcMeta<TCounter: RcCounter> {
    /// The pointer to the vector.
    pub vec_ptr: *const u8,
    /// the capacity of the vector.
    pub capacity: usize,
    /// the reference counter.
    pub counter: TCounter,
}

impl<TCounter: RcCounter> RcMeta<TCounter> {
    pub fn initial(counter: TCounter) -> Self {
        Self {
            vec_ptr: ptr::null(),
            capacity: 0,
            counter,
        }
    }

    /// extracts the embedded vector.
    #[inline]
    pub unsafe fn extract_vec(&self, len: usize) -> Vec<u8> {
        Vec::from_raw_parts(self.vec_ptr as *mut u8, len, self.capacity)
    }
}