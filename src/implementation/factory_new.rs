use std::marker::PhantomData;

use crate::{AnyRc, Bin, DefaultExcessShrink, EmptyBin, ExcessShrink, Factory, IntoUnSyncView, maybe_shrink, RcBin, StaticBin, VecBin, StackBin};

pub struct New {
    _phantom: PhantomData<()>
}

impl Factory for New {
    type T = Bin;

    #[inline]
    fn empty() -> Self::T {
        EmptyBin::new().un_sync()
    }

    #[inline]
    fn from_static(slice: &'static [u8]) -> Self::T {
        StaticBin::from(slice).un_sync()
    }

    #[inline]
    fn copy_from_slice(slice: &[u8]) -> Self::T {
        if let Some(stack_bin) = StackBin::try_from(slice) {
            stack_bin.un_sync()
        } else {
            RcBin::copy_from_slice(slice)
        }
    }

    #[inline]
    fn from_iter(iter: impl IntoIterator<Item=u8>) -> Self::T {
        RcBin::from_iter(iter)
    }

    #[inline]
    fn vec_excess() -> usize {
        RcBin::overhead_bytes()
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
            RcBin::from_vec(vec)
        } else {
            // not enough excess, use a vector instead
            VecBin::from_vec(vec, false).un_sync()
        }
    }
}