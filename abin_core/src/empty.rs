use abin_interface::{Bin, BinConfig, BinData, SyncBin, UnsafeBin};

/// A binary that's always empty.
pub struct EmptyBin;

impl EmptyBin {
    pub fn new() -> SyncBin {
        unsafe { Bin::_new(BinData(0, 0, 0), &CONFIG)._into_sync() }
    }
}

const CONFIG: BinConfig = BinConfig {
    drop,
    as_slice,
    is_empty,
};

fn drop(_: &mut Bin) {}

fn as_slice(_: &Bin) -> &[u8] {
    &[]
}

fn is_empty(_: &Bin) -> bool {
    true
}