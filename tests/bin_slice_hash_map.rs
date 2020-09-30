use std::collections::HashMap;
use abin::{StaticBin, Bin, SyncBin};

/// binary is compatible with hash map.
#[test]
fn use_bin_slice_in_hash_map() {
    let mut map = HashMap::<Bin, String>::default();

    map.insert(StaticBin::from(&[]).un_sync(), "empty".to_owned());
    map.insert(StaticBin::from(&[4]).un_sync(), "just 4".to_owned());
    map.insert(StaticBin::from(&[58, 4]).un_sync(), "58 and 4".to_owned());

    assert_eq!(map.get(&[] as &[u8]), Some(&"empty".to_owned()));
    assert_eq!(map.get(&[4u8] as &[u8]), Some(&"just 4".to_owned()));
    assert_eq!(map.get(&[58u8, 4u8] as &[u8]), Some(&"58 and 4".to_owned()));
}

/// binary is compatible with hash map.
#[test]
fn use_sync_bin_slice_in_hash_map() {
    let mut map = HashMap::<SyncBin, String>::default();

    map.insert(StaticBin::from(&[]), "empty".to_owned());
    map.insert(StaticBin::from(&[4]), "just 4".to_owned());
    map.insert(StaticBin::from(&[58, 4]), "58 and 4".to_owned());

    assert_eq!(map.get(&[] as &[u8]), Some(&"empty".to_owned()));
    assert_eq!(map.get(&[4u8] as &[u8]), Some(&"just 4".to_owned()));
    assert_eq!(map.get(&[58u8, 4u8] as &[u8]), Some(&"58 and 4".to_owned()));
}