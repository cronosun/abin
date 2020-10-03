use crate::{
    maybe_shrink, AnyBin, AnyRc, ArcBin, Bin, DefaultExcessShrink, EmptyBin, ExcessShrink, Factory,
    IntoUnSyncView, New, RcBin, SBin, SNew, StackBin, StaticBin, VecBin,
};

pub trait CommonFactory {
    type TAnyRc: AnyRc;
    type TSyncToUnSyncConverter: SyncToUnSyncConverter<
        TSync = SBin,
        TUnSync = <Self::TAnyRc as AnyRc>::T,
    >;
}

pub trait SyncToUnSyncConverter {
    type TSync;
    type TUnSync;
    fn convert_to_un_sync(value: Self::TSync) -> Self::TUnSync;
}

impl<TCf> Factory for TCf
where
    TCf: CommonFactory,
    <TCf::TAnyRc as AnyRc>::T: AnyBin,
{
    type T = <TCf::TAnyRc as AnyRc>::T;

    #[inline]
    fn empty() -> Self::T {
        TCf::TSyncToUnSyncConverter::convert_to_un_sync(EmptyBin::new())
    }

    #[inline]
    fn from_static(slice: &'static [u8]) -> Self::T {
        TCf::TSyncToUnSyncConverter::convert_to_un_sync(StaticBin::from(slice))
    }

    #[inline]
    fn copy_from_slice(slice: &[u8]) -> Self::T {
        // use stack if it's small
        if let Some(stack_bin) = StackBin::try_from(slice) {
            TCf::TSyncToUnSyncConverter::convert_to_un_sync(stack_bin)
        } else {
            TCf::TAnyRc::copy_from_slice(slice)
        }
    }

    #[inline]
    fn from_iter(iter: impl IntoIterator<Item = u8>) -> Self::T {
        let iter = iter.into_iter();
        match StackBin::try_from_iter(iter) {
            Ok(stack) => TCf::TSyncToUnSyncConverter::convert_to_un_sync(stack),
            Err(iterator) => TCf::TAnyRc::from_iter(iterator),
        }
    }

    #[inline]
    fn vec_excess() -> usize {
        TCf::TAnyRc::overhead_bytes()
    }

    #[inline]
    fn from_vec(vec: Vec<u8>) -> Self::T {
        TCf::from_vec_reduce_excess::<DefaultExcessShrink>(vec)
    }

    #[inline]
    fn from_vec_reduce_excess<T: ExcessShrink>(mut vec: Vec<u8>) -> Self::T {
        maybe_shrink::<T>(&mut vec, Self::vec_excess());
        // here we just check whether there's sufficient excess
        let excess = vec.capacity() - vec.len();
        if excess >= Self::vec_excess() {
            // sufficient excess for reference-counting
            TCf::TAnyRc::from_vec(vec)
        } else {
            // that's not good... not enough excess, use a vector instead ... or stack
            if let Some(stack) = StackBin::try_from(vec.as_slice()) {
                TCf::TSyncToUnSyncConverter::convert_to_un_sync(stack)
            } else {
                TCf::TSyncToUnSyncConverter::convert_to_un_sync(VecBin::from_vec(vec, false))
            }
        }
    }
}

impl CommonFactory for New {
    type TAnyRc = RcBin;
    type TSyncToUnSyncConverter = SyncToUnSyncConverterForNew;
}

pub struct SyncToUnSyncConverterForNew {}

impl SyncToUnSyncConverter for SyncToUnSyncConverterForNew {
    type TSync = SBin;
    type TUnSync = Bin;

    #[inline]
    fn convert_to_un_sync(value: Self::TSync) -> Self::TUnSync {
        value.un_sync()
    }
}

impl CommonFactory for SNew {
    type TAnyRc = ArcBin;
    type TSyncToUnSyncConverter = SyncToUnSyncConverterForSNew;
}

pub struct SyncToUnSyncConverterForSNew {}

impl SyncToUnSyncConverter for SyncToUnSyncConverterForSNew {
    type TSync = SBin;
    type TUnSync = SBin;

    #[inline]
    fn convert_to_un_sync(value: Self::TSync) -> Self::TUnSync {
        // does nothing
        value
    }
}
