use core::mem;
use std::marker::PhantomData;

use crate::{
    Bin, DefaultVecCapShrink, FnTable, NoVecCapShrink, NsRcCounter, RcCounter, RcData, StackBin,
    SyncRcCounter, UnsafeBin, VecCapShrink,
};

pub struct AnyRcImpl<TConfig: AnyRcImplConfig> {
    _phantom: PhantomData<TConfig>,
}

impl<TConfig: AnyRcImplConfig> AnyRcImpl<TConfig> {
    #[inline]
    pub(crate) fn from_vec(vec: Vec<u8>) -> Bin {
        Self::from_with_cap_shrink::<DefaultVecCapShrink>(vec)
    }

    #[inline]
    pub(crate) fn from_with_cap_shrink<T: VecCapShrink>(mut vec: Vec<u8>) -> Bin {
        if let Some(stack) = StackBin::try_from(vec.as_slice()) {
            stack.un_sync()
        } else {
            RcData::<TConfig::TCounter>::maybe_shrink_vec::<T>(&mut vec);
            let rc_data = unsafe { RcData::<TConfig::TCounter>::new_from_vec_raw(vec) };
            unsafe { Bin::_new(rc_data.to_bin_data(), TConfig::table()) }
        }
    }

    #[inline]
    pub(crate) fn copy_from_slice(slice: &[u8]) -> Bin {
        if let Some(stack) = StackBin::try_from(slice) {
            stack.un_sync()
        } else {
            let vec = RcData::<TConfig::TCounter>::slice_to_vec_with_meta_overhead(slice);
            // we do not need capacity shrink (vector should already be ok).
            Self::from_with_cap_shrink::<NoVecCapShrink>(vec)
        }
    }

    #[inline]
    pub(crate) fn overhead_bytes() -> usize {
        RcData::<TConfig::TCounter>::meta_overhead()
    }
}

pub struct AnyRcConfigForNonSync;

impl AnyRcImplConfig for AnyRcConfigForNonSync {
    type TCounter = NsRcCounter;

    #[inline]
    fn table() -> &'static FnTable {
        &NON_SYNC_FN_TABLE
    }
}

pub struct AnyRcConfigForSync;

impl AnyRcImplConfig for AnyRcConfigForSync {
    type TCounter = SyncRcCounter;

    #[inline]
    fn table() -> &'static FnTable {
        &SYNC_FN_TABLE
    }
}

pub trait AnyRcImplConfig {
    type TCounter: RcCounter + 'static;
    fn table() -> &'static FnTable;
}

const NON_SYNC_FN_TABLE: FnTable = FnTable {
    drop: drop::<NsRcCounter>,
    as_slice: as_slice::<NsRcCounter>,
    is_empty: is_empty::<NsRcCounter>,
    clone: clone::<NsRcCounter>,
    into_vec: into_vec::<NsRcCounter>,
    slice: slice::<NsRcCounter>,
};

const SYNC_FN_TABLE: FnTable = FnTable {
    drop: drop::<SyncRcCounter>,
    as_slice: as_slice::<SyncRcCounter>,
    is_empty: is_empty::<SyncRcCounter>,
    clone: clone::<SyncRcCounter>,
    into_vec: into_vec::<SyncRcCounter>,
    slice: slice::<SyncRcCounter>,
};

fn drop<TCounter: RcCounter>(bin: &mut Bin) {
    let rc_data = unsafe { RcData::<TCounter>::from_bin_mut(bin) };
    rc_data.drop();
}

fn as_slice<TCounter: RcCounter + 'static>(bin: &Bin) -> &[u8] {
    let rc_data = unsafe { RcData::<TCounter>::from_bin(bin) };
    rc_data.as_slice()
}

fn is_empty<TCounter: RcCounter>(bin: &Bin) -> bool {
    let rc_data = unsafe { RcData::<TCounter>::from_bin(bin) };
    rc_data.is_empty()
}

fn clone<TCounter: RcCounter>(bin: &Bin) -> Bin {
    let rc_data = unsafe { RcData::<TCounter>::from_bin_mut_cast(bin) };
    let rc_data = rc_data.clone();
    unsafe { Bin::_new(rc_data.into_bin_data(), bin._fn_table()) }
}

fn into_vec<TCounter: RcCounter>(mut bin: Bin) -> Vec<u8> {
    let rc_data = unsafe { RcData::<TCounter>::from_bin_mut(&mut bin) };
    let vec = rc_data.into_vec();
    // bin must not be dropped (we still might need the content)
    mem::forget(bin);
    vec
}

fn slice<TCounter: RcCounter>(bin: &Bin, start: usize, end_excluded: usize) -> Option<Bin> {
    let rc_data = unsafe { RcData::<TCounter>::from_bin_mut_cast(bin) };
    if let Some(rc_data) = rc_data.slice(start, end_excluded) {
        Some(unsafe { Bin::_new(rc_data.into_bin_data(), bin._fn_table()) })
    } else {
        None
    }
}
