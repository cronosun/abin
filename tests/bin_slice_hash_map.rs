use abin::{Bin, Factory, New, SBin, SNew};
use std::collections::HashMap;

/// binary is compatible with hash map.
#[test]
fn use_bin_slice_in_hash_map() {
    let mut map = HashMap::<Bin, String>::default();

    map.insert(New::from_static(&[]), "empty".to_owned());
    map.insert(New::from_static(&[4]), "just 4".to_owned());
    map.insert(New::from_static(&[58, 4]), "58 and 4".to_owned());

    assert_eq!(map.get(&[] as &[u8]), Some(&"empty".to_owned()));
    assert_eq!(map.get(&[4u8] as &[u8]), Some(&"just 4".to_owned()));
    assert_eq!(map.get(&[58u8, 4u8] as &[u8]), Some(&"58 and 4".to_owned()));
}

/// binary is compatible with hash map.
#[test]
fn use_sync_bin_slice_in_hash_map() {
    let mut map = HashMap::<SBin, String>::default();

    map.insert(SNew::from_static(&[]), "empty".to_owned());
    map.insert(SNew::from_static(&[4]), "just 4".to_owned());
    map.insert(SNew::from_static(&[58, 4]), "58 and 4".to_owned());

    assert_eq!(map.get(&[] as &[u8]), Some(&"empty".to_owned()));
    assert_eq!(map.get(&[4u8] as &[u8]), Some(&"just 4".to_owned()));
    assert_eq!(map.get(&[58u8, 4u8] as &[u8]), Some(&"58 and 4".to_owned()));
}
