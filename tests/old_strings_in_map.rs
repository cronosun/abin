use abin::{NewSStr, NewStr, SStr, Str, StrFactory};
use std::collections::HashMap;

/// string is compatible with hash map.
#[test]
fn use_string_in_hash_map() {
    let mut map = HashMap::<Str, String>::default();

    map.insert(NewStr::from_static("entry 1"), "hello".to_owned());
    map.insert(NewStr::from_static("other_entry"), "world".to_owned());
    map.insert(NewStr::from_given_string("daa".to_owned()), "ok".to_owned());

    assert_eq!(map.get("entry 1"), Some(&"hello".to_owned()));
    assert_eq!(map.get("other_entry"), Some(&"world".to_owned()));
    assert_eq!(map.get("daa"), Some(&"ok".to_owned()));
}

/// string is compatible with hash map.
#[test]
fn use_sync_string_in_hash_map() {
    let mut map = HashMap::<SStr, String>::default();

    map.insert(NewSStr::from_static("entry 1"), "hello".to_owned());
    map.insert(
        NewSStr::from_given_string("other_entry".to_owned()),
        "world".to_owned(),
    );
    map.insert(
        NewSStr::from_given_string("daa".to_owned()),
        "ok".to_owned(),
    );

    assert_eq!(map.get("entry 1"), Some(&"hello".to_owned()));
    assert_eq!(map.get("other_entry"), Some(&"world".to_owned()));
    assert_eq!(map.get("daa"), Some(&"ok".to_owned()));
}
