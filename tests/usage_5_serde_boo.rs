use std::borrow::Cow;
use serde::{Serialize, Deserialize};
use abin::{BooStr, BooVec};

/// Serialize/de-serialize using serde with Boo (like Cow).
#[test]
fn serialize_deserialize_using_serde() {
    let entity = create_entity();
    let as_vec = serde_cbor::to_vec(&entity).unwrap();

    let restored_entity = de_serialize_from_slice(as_vec.as_slice());

    assert_eq!(entity, restored_entity);
}

fn de_serialize_from_slice(slice : &[u8]) -> MyEntity {
    serde_cbor::from_slice(slice).unwrap()
}

fn create_entity() -> MyEntity<'static> {
    MyEntity {
        person_list: Cow::Borrowed(&[
            Person {
                first_name: "Elbort".into(),
                last_name: "Zweistein".into(),
            },
            Person {
                first_name: "Lutz".into(),
                last_name: "Roeder".into(),
            },
            Person {
                first_name: "Franz".into(),
                last_name: "Taxi".into(),
            }
        ])
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
struct MyEntity<'a> {
    #[serde(borrow="'a")]
    person_list: Cow<'a, [Person<'a>]>
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
struct Person<'a> {
    #[serde(borrow="'a")]
    first_name: BooStr<'a>,
    #[serde(borrow="'a")]
    last_name: BooStr<'a>,
}