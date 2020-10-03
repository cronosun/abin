use serde::export::PhantomData;

use crate::{AnyBin, AnyRc, Bin, DefaultExcessShrink, EmptyBin, ExcessShrink, Factory, maybe_shrink, SBin, StackBin, StaticBin, VecBin};

pub struct CommonFactory<TAnyRc, TSyncToUnSyncConverter> {
    _phantom1: PhantomData<TAnyRc>,
    _phantom2: PhantomData<TSyncToUnSyncConverter>,
}

pub trait SyncToUnSyncConverter {
    type TSync;
    type TUnSync;
    fn convert_to_un_sync(value: Self::TSync) -> Self::TUnSync;
}

impl<TAnyRc, TSyncToUnSyncConverter> Factory for CommonFactory<TAnyRc, TSyncToUnSyncConverter>
    where TAnyRc: AnyRc, TSyncToUnSyncConverter: SyncToUnSyncConverter<TSync=SBin, TUnSync=TAnyRc::T>, TAnyRc::T: AnyBin,
{
    type T = TAnyRc::T;

    #[inline]
    fn empty() -> Self::T {
        TSyncToUnSyncConverter::convert_to_un_sync(EmptyBin::new())
    }

    #[inline]
    fn from_static(slice: &'static [u8]) -> Self::T {
        TSyncToUnSyncConverter::convert_to_un_sync(StaticBin::from(slice))
    }

    #[inline]
    fn copy_from_slice(slice: &[u8]) -> Self::T {
        // use stack if it's small
        if let Some(stack_bin) = StackBin::try_from(slice) {
            TSyncToUnSyncConverter::convert_to_un_sync(stack_bin)
        } else {
            TAnyRc::copy_from_slice(slice)
        }
    }

    #[inline]
    fn from_iter(iter: impl IntoIterator<Item=u8>) -> Self::T {
        let iter = iter.into_iter();
        match StackBin::try_from_iter(iter) {
            Ok(stack) => TSyncToUnSyncConverter::convert_to_un_sync(stack),
            Err(iterator) => {
                TAnyRc::from_iter(iterator)
            }
        }
    }

    #[inline]
    fn vec_excess() -> usize {
        TAnyRc::overhead_bytes()
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
            TAnyRc::from_vec(vec)
        } else {
            // that's not good... not enough excess, use a vector instead ... or stack
            if let Some(stack) = StackBin::try_from(vec.as_slice()) {
                TSyncToUnSyncConverter::convert_to_un_sync(stack)
            } else {
                TSyncToUnSyncConverter::convert_to_un_sync(VecBin::from_vec(vec, false))
            }
        }
    }
}