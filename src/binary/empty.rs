use crate::{Bin, FnTable, BinData, SyncBin};

/// A binary that's always empty.
pub struct EmptyBin;

impl EmptyBin {
    #[inline]
    pub const fn new() -> SyncBin {
        SyncBin(Bin::_const_new(BinData(core::ptr::null(), 0, 0), &FN_TABLE))
    }
}

const FN_TABLE : FnTable = FnTable {
    drop,
    as_slice,
    is_empty,
    clone,
    into_vec,
};

fn drop(_: &mut Bin) {}

fn as_slice(_: &Bin) -> &[u8] {
    &[]
}

fn is_empty(_: &Bin) -> bool {
    true
}

fn clone(_: &Bin) -> Bin {
    EmptyBin::new().un_sync()
}

fn into_vec(_: Bin) -> Vec<u8> {
    Vec::new()
}