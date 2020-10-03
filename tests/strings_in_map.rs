use abin::{Str, SStr};
use std::collections::HashMap;

/// string is compatible with hash map.
#[test]
fn use_string_in_hash_map() {
    let mut map = HashMap::<Str, String>::default();

    map.insert("entry 1".into(), "hello".to_owned());
    map.insert("other_entry".to_owned().into(), "world".to_owned());
    map.insert("daa".to_owned().into(), "ok".to_owned());

    assert_eq!(map.get("entry 1"), Some(&"hello".to_owned()));
    assert_eq!(map.get("other_entry"), Some(&"world".to_owned()));
    assert_eq!(map.get("daa"), Some(&"ok".to_owned()));
}

/// string is compatible with hash map.
#[test]
fn use_sync_string_in_hash_map() {
    let mut map = HashMap::<SStr, String>::default();

    map.insert("entry 1".into(), "hello".to_owned());
    map.insert("other_entry".to_owned().into(), "world".to_owned());
    map.insert("daa".to_owned().into(), "ok".to_owned());

    assert_eq!(map.get("entry 1"), Some(&"hello".to_owned()));
    assert_eq!(map.get("other_entry"), Some(&"world".to_owned()));
    assert_eq!(map.get("daa"), Some(&"ok".to_owned()));
}
