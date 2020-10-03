use std::marker::PhantomData;

use crate::{AnyRc, ArcBin, Bin, DefaultExcessShrink, EmptyBin, ExcessShrink, Factory, IntoUnSyncView, maybe_shrink, RcBin, SBin, StaticBin, VecBin, StackBin};

pub struct SNew {
    _phantom: PhantomData<()>
}

impl Factory for SNew {
    type T = SBin;

    #[inline]
    fn empty() -> Self::T {
        EmptyBin::new()
    }

    #[inline]
    fn from_static(slice: &'static [u8]) -> Self::T {
        StaticBin::from(slice)
    }

    #[inline]
    fn copy_from_slice(slice: &[u8]) -> Self::T {
        if let Some(stack_bin) = StackBin::try_from(slice) {
            stack_bin
        } else {
            ArcBin::copy_from_slice(slice)
        }
    }

    #[inline]
    fn from_iter(iter: impl IntoIterator<Item=u8>) -> Self::T {
        ArcBin::from_iter(iter)
    }

    #[inline]
    fn vec_excess() -> usize {
        ArcBin::overhead_bytes()
    }

    #[inline]
    fn from_vec(vec: Vec<u8>) -> Self::T {
        Self::from_vec_reduce_excess::<DefaultExcessShrink>(vec)
    }

    #[inline]
    fn from_vec_reduce_excess<T: ExcessShrink>(mut vec: Vec<u8>) -> Self::T {
        maybe_shrink::<T>(&mut vec, Self::vec_excess());
        // here we just check whether there's sufficient excess
        let excess = vec.capacity() - vec.len();
        if excess >= Self::vec_excess() {
            // sufficient excess for reference-counting
            ArcBin::from_vec(vec)
        } else {
            // not enough excess, use a vector instead
            VecBin::from_vec(vec, false)
        }
    }
}