use std::collections::HashMap;
use std::ops::Deref;

use abin::{BooStr, NewStr, Str, StrFactory, ToBooConverter};

#[test]
fn entity_with_borrowed_or_owned_fields() {
    let first_name = NewStr::copy_from_str("Hans");
    let first_name_clone = first_name.clone();
    let last_name = NewStr::from_static("Muster");

    let entity_borrowed_first_name = StrEntity {
        // using ToBooConverter::borrowed()
        first_name: first_name.borrowed(),
        // using From / Into (always borrowed)
        last_name: "Muster".into(),
    };
    let entity_owned_first_name = StrEntity {
        // using ToBooConverter::owned()
        first_name: first_name_clone.owned(),
        // using ToBooConverter::borrowed()
        last_name: last_name.borrowed(),
    };

    assert_eq!(
        first_name,
        extract_first_name_owned(entity_borrowed_first_name, "Muster")
    );
    assert_eq!(
        first_name,
        extract_first_name_owned(entity_owned_first_name, "Muster")
    );
}

fn extract_first_name_owned<'a>(
    entity: StrEntity,
    expected_last_name: impl Into<BooStr<'a>>,
) -> Str {
    assert_eq!(entity.last_name.deref(), expected_last_name.into().deref());
    // Convert a Boo (BooStr in this case) into owned (`NewStr` is the implementation used to
    // convert "&str" to "Str" if `first_name` is borrowed).
    entity.first_name.into_owned_with::<NewStr>()
}

struct StrEntity<'a> {
    first_name: BooStr<'a>,
    last_name: BooStr<'a>,
}

#[test]
fn boo_in_hash_map() {
    // static entries: always owned.
    let mut map = HashMap::<BooStr<'static>, BooStr<'static>>::new();

    map.insert(
        NewStr::from_static("entry1").owned(),
        NewStr::from_static("Value for entry1").owned(),
    );
    map.insert(
        NewStr::from_static("entry2").owned(),
        NewStr::from_static("Value for entry2").owned(),
    );
    map.insert(
        NewStr::from_static("entry3").owned(),
        NewStr::from_static("Value for entry3").owned(),
    );

    // query the map. Can just use normal "&str".
    assert_eq!(map.get("entry1").unwrap().deref(), "Value for entry1");
    assert_eq!(map.get("entry2").unwrap().deref(), "Value for entry2");
    assert_eq!(map.get("entry3").unwrap().deref(), "Value for entry3");
}
