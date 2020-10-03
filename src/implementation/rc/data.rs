use core::{mem, slice};

use crate::{
    Bin, BinData, DefaultExcessShrink, RcCounter, RcDecResult, RcMeta, RcUtils, UnsafeBin,
};

#[repr(C)]
pub struct RcData<TCounter: RcCounter> {
    /// pointer to the data. Note: This is not always the same as the vector data (if you
    /// slice the rc-data this might differ).
    pub data_ptr: *const u8,
    /// the length of the data. Note: This is not the same as the vector data capacity.
    pub data_len: usize,
    /// pointer to the meta-data.
    pub meta_ptr: *const RcMeta<TCounter>,
}

impl<TCounter: RcCounter> RcData<TCounter> {
    #[inline]
    pub unsafe fn from_bin(bin: &Bin) -> &Self {
        let bin_data = bin._data() as *const BinData;
        let self_data = mem::transmute::<*const BinData, *const Self>(bin_data);
        &*self_data
    }

    #[inline]
    pub unsafe fn from_bin_mut_cast(bin: &Bin) -> &mut Self {
        let bin_data = bin._data() as *const BinData as *mut BinData;
        let self_data = mem::transmute::<*mut BinData, *mut Self>(bin_data);
        &mut *self_data
    }

    #[inline]
    pub unsafe fn from_bin_mut(bin: &mut Bin) -> &mut Self {
        let bin_data = bin._data_mut() as *mut BinData;
        let self_data = mem::transmute::<*mut BinData, *mut Self>(bin_data);
        &mut *self_data
    }

    #[inline]
    pub unsafe fn to_bin_data(&self) -> BinData {
        mem::transmute_copy::<Self, BinData>(self)
    }

    #[inline]
    pub unsafe fn into_bin_data(self) -> BinData {
        mem::transmute::<Self, BinData>(self)
    }
}

impl<TCounter: RcCounter> RcData<TCounter> {
    /// New: Does not shrink the vector and does not use optimization (EmptyBin or StackBin).
    #[inline]
    pub(crate) unsafe fn new_from_vec_raw(mut vec: Vec<u8>) -> Self {
        let meta = RcMeta::<TCounter>::initial(TCounter::new());
        let meta = RcUtils::add_padding_and_metadata(&mut vec, meta);
        // setup meta data
        let meta_mut = meta as *mut RcMeta<TCounter>;
        (*meta_mut).capacity = vec.capacity();
        (*meta_mut).vec_ptr = vec.as_ptr();

        let this = Self {
            // for non-sliced versions, this is just the same as the vector itself.
            data_ptr: vec.as_ptr(),
            // for non-sliced versions, this is just the same as the vector itself.
            data_len: vec.len(),
            meta_ptr: meta,
        };
        // must not free the vec (we still need its content).
        mem::forget(vec);
        this
    }

    #[inline]
    fn rc_meta(&self) -> &RcMeta<TCounter> {
        let meta_ptr = self.meta_ptr;
        unsafe { &*meta_ptr }
    }

    #[inline]
    fn rc_meta_mut(&mut self) -> &mut RcMeta<TCounter> {
        let meta_ptr = self.meta_ptr as *mut RcMeta<TCounter>;
        unsafe { &mut *meta_ptr }
    }

    #[inline]
    pub(crate) fn drop(&mut self) {
        let meta = self.rc_meta_mut();
        let dec_result = meta.counter.decrement();
        match dec_result {
            RcDecResult::Zero => {
                // last reference, free the vector: for this we just get the original vector back.
                // This will drop it immediately and thus free the memory. note: the length does
                // not matter, since it's all u8 (and u8 does not implement drop).
                unsafe { meta.extract_vec(0) };
            }
            RcDecResult::More => {
                // nothing to do here
            }
        }
    }

    #[inline]
    pub(crate) fn clone(&mut self) -> Self {
        self.rc_meta_mut().counter.increment();
        Self {
            data_ptr: self.data_ptr,
            meta_ptr: self.meta_ptr,
            data_len: self.data_len,
        }
    }

    #[inline]
    pub(crate) fn as_slice(&self) -> &[u8] {
        let ptr = self.data_ptr;
        let len = self.data_len;
        unsafe { slice::from_raw_parts(ptr, len) }
    }

    #[inline]
    pub(crate) fn is_empty(&self) -> bool {
        self.data_len == 0
    }

    /// note: when calling this, make sure the `Bin` is not dropped (since we still need the content).
    #[inline]
    pub(crate) fn into_vec(&mut self) -> Vec<u8> {
        // this can be a no-op (if it's not shared).
        let dec_result = {
            let meta = self.rc_meta_mut();
            meta.counter.decrement()
        };
        match dec_result {
            RcDecResult::Zero => {
                let meta = self.rc_meta();
                // great, it's the last one, maybe we can use the vector. We can use the
                // vector if this is not a sliced-rc with a different start offset.
                if self.data_ptr == meta.vec_ptr {
                    // great, looks good, we can use that vector
                    let mut vec = unsafe { meta.extract_vec(self.data_len) };
                    // we also shrink the vector, why? If this is a slice, the vector might be
                    // way too large.
                    RcUtils::maybe_shrink_vec::<TCounter, DefaultExcessShrink>(&mut vec);
                    vec
                } else {
                    // no, unfortunately we can't use that vector. but extract it anyway, so
                    // it gets dropped.
                    let vec = unsafe { meta.extract_vec(self.data_len) };
                    let new_vec =
                        RcUtils::slice_to_vec_with_meta_overhead::<TCounter>(vec.as_slice());
                    new_vec
                }
            }
            RcDecResult::More => {
                // we definitely have to copy, there's still references.
                let slice = self.as_slice();
                RcUtils::slice_to_vec_with_meta_overhead::<TCounter>(slice)
            }
        }
    }

    /// It's basically the same as clone with some adjustments.
    #[inline]
    pub(crate) fn slice(&mut self, start: usize, end_excluded: usize) -> Option<Self> {
        if self.as_slice().get(start..end_excluded).is_some() {
            // ok, within range
            let mut clone = self.clone();
            unsafe {
                clone.data_ptr = clone.data_ptr.add(start);
            }
            clone.data_len = end_excluded - start;
            Some(clone)
        } else {
            None
        }
    }
}
