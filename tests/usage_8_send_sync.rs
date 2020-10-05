use abin::{AnyBin, Bin, BinFactory, IntoSync, IntoUnSync, IntoUnSyncView, NewBin, NewSBin, SBin};

/// Converts something non-sync to sync.
#[test]
fn convert_to_send_sync() {
    // that's already send + sync (`SBin`)
    let sync: SBin = NewSBin::copy_from_slice("Initially sync".as_bytes());
    something_that_requires_binary_to_be_send_and_sync(sync, "Initially sync".as_bytes());

    // this we have to convert first (got something non-sync from an external module)
    let non_sync_got_externally: Bin = NewBin::copy_from_slice("Something non-sync".as_bytes());
    let converted_to_sync: SBin = non_sync_got_externally.into_sync();
    something_that_requires_binary_to_be_send_and_sync(
        converted_to_sync,
        "Something non-sync".as_bytes(),
    );
}

/// There's two possibilities to convert something sync to non-sync.
#[test]
fn convert_back_to_non_sync() {
    // Possibility 1: Just wrap the sync binary inside a non-sync view. This is a fast conversion
    // but the binary still has the overhead of synchronization.
    let sync1: SBin = NewSBin::copy_from_slice("Initially sync 1".as_bytes());
    let non_sync1 = sync1.un_sync();
    assert_eq!("Initially sync 1".as_bytes(), non_sync1.as_slice());

    // Possibility 2: Perform a real conversion. This is (under some circumstances) somewhat
    // expensive - usually it's cheap too. "some circumstances": When you convert a reference-counted
    // binary from sync to non-sync and this binary has other references pointing to it (in this
    // case this creates a copy of that binary).
    let sync2: SBin = NewSBin::copy_from_slice("Initially sync 2".as_bytes());
    let non_sync2 = sync2.un_sync_convert();
    assert_eq!("Initially sync 2".as_bytes(), non_sync2.as_slice());
}

fn something_that_requires_binary_to_be_send_and_sync<T: AnyBin + Send + Sync>(
    bin: T,
    expected: &[u8],
) {
    assert_eq!(expected, bin.as_slice());
}
