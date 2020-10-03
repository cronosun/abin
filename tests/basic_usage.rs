use abin::{AnyBin, Bin, Factory, IntoUnSyncView, New, SNew};

#[test]
pub fn usage() {
    // empty binary, stack-only.
    let bin1 = New::empty();
    // small binary; stack-only.
    let bin2 = New::copy_from_slice(&[5, 10]);
    // reference-counted binary (not synchronized);
    let bin3 = New::copy_from_slice("This is a binary; too large for the stack.".as_bytes());
    // reference-counted binary (synchronized);
    let bin4 = SNew::from_vec(
        "This is a binary; too large for the stack."
            .to_owned()
            .into_bytes(),
    );
    // no allocation for static data.
    let bin5 = New::from_static("Static data".as_bytes());

    use_bin(bin1);
    use_bin(bin2);
    use_bin(bin3);
    // 'un_sync' is a cheap operation that converts SyncBin to Bin. You can also use `Into` instead.
    use_bin(bin4.un_sync());
    use_bin(bin5);
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
