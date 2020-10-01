use abin::{Bin, SyncBin};

#[test]
fn bin_size() {
    // should have a size of 4 words
    let word_size = core::mem::size_of::<usize>();
    let bin_size = core::mem::size_of::<Bin>();
    assert_eq!(word_size * 4, bin_size)
}

#[test]
fn sync_bin_size() {
    // should have a size of 4 words
    let word_size = core::mem::size_of::<usize>();
    let sync_bin_size = core::mem::size_of::<SyncBin>();
    assert_eq!(word_size * 4, sync_bin_size);
    assert_eq!(core::mem::size_of::<Bin>(), sync_bin_size);
}

#[test]
fn bin_align() {
    let word_align = core::mem::align_of::<usize>();
    let bin_align = core::mem::align_of::<Bin>();
    assert_eq!(word_align, bin_align)
}

#[test]
fn sync_bin_align() {
    let word_align = core::mem::align_of::<usize>();
    let bin_align = core::mem::align_of::<SyncBin>();
    assert_eq!(word_align, bin_align)
}
