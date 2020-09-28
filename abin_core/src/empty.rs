use abin_interface::{Bin, BinConfig, BinData, SyncBin, UnsafeBin};

/// A binary that's always empty.
pub struct EmptyBin;

impl EmptyBin {
    #[inline]
    pub fn new() -> SyncBin {
        unsafe { Bin::_new(BinData(0, 0, 0), &CONFIG)._into_sync() }
    }
}

const CONFIG: BinConfig = BinConfig {
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