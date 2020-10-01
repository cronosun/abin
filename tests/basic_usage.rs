use abin::{AnyBin, AnyRc, ArcBin, Bin, EmptyBin, IntoUnSyncView, RcBin, StaticBin, VecBin};

#[test]
pub fn usage() {
    // empty binary, stack-only.
    let bin1 = EmptyBin::new();
    // small binary; stack-only.
    let bin2 = RcBin::copy_from_slice(&[5, 10]);
    // reference-counted binary (not synchronized); from a slice; can also be constructed from a vec.
    let bin3 = RcBin::copy_from_slice("This is a binary; too large for the stack.".as_bytes());
    // reference-counted binary (synchronized); this time from a vector (does not allocate if the
    // vector has enough capacity for the meta-data).
    let bin4 = ArcBin::from_vec(
        "This is a binary; too large for the stack."
            .to_owned()
            .into_bytes(),
    );
    // binary backed by a Vec<u8>.
    let bin5 = VecBin::from_vec(
        "This is a vector binary, backed by a vector"
            .to_owned()
            .into_bytes(),
        true,
    );
    // no allocation for static data.
    let bin6 = StaticBin::from("Static data".as_bytes());

    // 'un_sync' is a cheap operation that converts SyncBin to Bin. You can also use `Into` instead.
    use_bin(bin1.un_sync());
    use_bin(bin2);
    use_bin(bin3);
    use_bin(bin4.un_sync());
    use_bin(bin5.un_sync());
    use_bin(bin6.un_sync());
}

/// Just two interfaces for all binaries (`Bin`/`SyncBin`) - `SyncBin` can be converted to `Bin`.
pub fn use_bin(bin: Bin) {
    // length of the binary (cheap operation).
    let len = bin.len();
    // to &[u8] (cheap operation)
    let _u8_slice = bin.as_slice();
    // can be cloned (for reference-counted binaries, StaticBin and stack-binary, this is cheap).
    let cloned_bin = bin.clone();
    assert_eq!(bin, cloned_bin);
    assert_eq!(len, cloned_bin.len());
    // can be sliced (cheap operation for reference-counted binaries, StaticBin and stack-binary).
    let slice = bin.slice(0..10);
    if let Some(slice) = slice {
        assert_eq!(10, slice.len());
    }
    // ...and converted into vector (cheap operation for VecBin and for reference-counted
    // binaries with no more references).
    let vec = bin.into_vec();
    assert_eq!(cloned_bin.as_slice(), vec.as_slice());
}
