use core::mem;
use std::iter::FromIterator;

use crate::{maybe_shrink, ExcessShrink, RcCounter, RcMeta, SizeHintExtendingIter};

pub struct RcUtils;

impl RcUtils {
    /// Creates a vector with sufficient capacity for the meta-data from given iterator.
    #[inline]
    pub(crate) fn vec_with_capacity_for_rc_from_iter<TCounter: RcCounter, TIter>(
        iter: TIter,
    ) -> Vec<u8>
    where
        TIter: IntoIterator<Item = u8>,
    {
        let extend_size_by = Self::meta_overhead::<TCounter>();
        let new_iter = SizeHintExtendingIter::new(iter.into_iter(), extend_size_by);
        Vec::from_iter(new_iter)
    }

    /// Shrinks the vector (if T says to do so) but sill keeps enough capacity for the metadata.
    #[inline]
    pub fn maybe_shrink_vec<TCounter: RcCounter, T: ExcessShrink>(vec: &mut Vec<u8>) {
        let overhead = Self::meta_overhead::<TCounter>();
        maybe_shrink::<T>(vec, overhead);
    }

    /// adds padding and metadata but without altering the vector len. Returns a pointer to the
    /// metadata.
    #[inline]
    pub(crate) unsafe fn add_padding_and_metadata<TCounter: RcCounter>(
        vec: &mut Vec<u8>,
        meta: RcMeta<TCounter>,
    ) -> *const RcMeta<TCounter> {
        Self::add_padding_and_metadata_inner::<TCounter>(vec, meta, true)
    }

    #[inline]
    unsafe fn add_padding_and_metadata_inner<TCounter: RcCounter>(
        vec: &mut Vec<u8>,
        meta: RcMeta<TCounter>,
        first_try: bool,
    ) -> *const RcMeta<TCounter> {
        // this can be a bit tricky... if we add capacity this might change the vector address ...
        // but changing the address can change required padding and padding is required to know
        // capacity. ... thus we just assume there's enough capacity...

        let len = vec.len();
        let meta_size = mem::size_of::<RcMeta<TCounter>>();

        // case A: assume there's enough capacity
        {
            let vec_ptr = vec.as_ptr();
            let meta_ptr_candidate = vec_ptr.add(vec.len());
            let required_padding =
                Self::padding(meta_ptr_candidate, mem::align_of::<RcMeta<TCounter>>());
            let additional_capacity_for_metadata = required_padding + meta_size;
            if vec.capacity() >= vec.len() + additional_capacity_for_metadata {
                // nice, we have enough capacity.
                let padding_buf = [0u8; 16];
                let padding_buf = &padding_buf[0..required_padding];
                vec.extend_from_slice(padding_buf);
                let meta_ptr =
                    meta_ptr_candidate.add(required_padding) as *mut u8 as *mut RcMeta<TCounter>;
                *meta_ptr = meta;
                // restore the vector length to the original length
                vec.set_len(len);
                return meta_ptr;
            }
        }

        if first_try {
            // case B: No, we have to increase the capacity.
            vec.reserve(Self::meta_overhead::<TCounter>());
            // now try again
            Self::add_padding_and_metadata_inner(vec, meta, false)
        } else {
            panic!(
                "Implementation error: We just reserved enough capacity but this method \
            tells us there's still not enough capacity."
            );
        }
    }

    pub(crate) fn slice_to_vec_with_meta_overhead<TCounter: RcCounter>(slice: &[u8]) -> Vec<u8> {
        let slice_len = slice.len();
        let mut vec = Vec::with_capacity(slice_len + Self::meta_overhead::<TCounter>());
        vec.extend_from_slice(slice);
        vec
    }

    /// Returns the additional bytes needed in a vector to store metadata. It's the maximum
    /// padding (worst case) required plus the size of the meta-data.
    #[inline]
    pub(crate) fn meta_overhead<TCounter: RcCounter>() -> usize {
        let alignment = mem::align_of::<RcMeta<TCounter>>();
        let size = mem::size_of::<RcMeta<TCounter>>();
        alignment - 1 + size
    }

    #[inline]
    fn padding(ptr: *const u8, alignment: usize) -> usize {
        let target = ptr as usize;
        let remainder = target % alignment;
        if remainder == 0 {
            // nice, already aligned
            0
        } else {
            let padding = alignment - remainder;
            assert!(padding <= (alignment - 1));
            padding
        }
    }
}
