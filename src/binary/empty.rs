use crate::{Bin, BinData, FnTable, IntoUnSyncView, SyncBin};

/// A binary that's always empty.
pub struct EmptyBin;

impl EmptyBin {
    #[inline]
    pub const fn new() -> SyncBin {
        SyncBin(Bin::_const_new(BinData::empty(), &FN_TABLE))
    }
}

const FN_TABLE: FnTable = FnTable {
    drop: None,
    as_slice: None,
    is_empty: None,
    clone,
    into_vec,
    slice,
    // not required: there's no non-synced version.
    convert_into_un_sync: None,
    // not required: this is already the sync version.
    convert_into_sync: None,
};

fn clone(_: &Bin) -> Bin {
    EmptyBin::new().un_sync()
}

fn into_vec(_: Bin) -> Vec<u8> {
    Vec::new()
}

fn slice(_: &Bin, start: usize, end_excluded: usize) -> Option<Bin> {
    if start == 0 && end_excluded == 0 {
        Some(EmptyBin::new().un_sync())
    } else {
        None
    }
}
