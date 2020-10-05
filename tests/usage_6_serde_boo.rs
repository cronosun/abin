use std::borrow::Cow;
use std::ops::Deref;

use serde::{Deserialize, Serialize};

use abin::{BooStr, BooVec};

/// Serialize/de-serialize using serde with Boo (like Cow).
#[test]
fn serialize_deserialize_using_serde<'a>() {
    let entity = create_entity();
    let as_vec = serde_cbor::to_vec(&entity).unwrap();
    step_1_server_got_data_from_network(as_vec);
}

fn step_1_server_got_data_from_network(data: Vec<u8>) {
    // now it's getting de-serialized
    let as_slice: &[u8] = data.as_slice();
    let data_for_server = de_serialize_from_slice(as_slice);
    step_2_server_processes_incoming_data(data_for_server);
}

fn step_2_server_processes_incoming_data(data: ServerCmdAddPeopleToAddressBook) {
    // here we can do something with data data
    for person in data.person_list.into_iter() {
        step_3_store_in_database(person);
    }
}

fn step_3_store_in_database(person: Person) {
    let first_name = person.first_name.deref();
    assert!(first_name == "Elbort" || first_name == "Lutz" || first_name == "Franz");
}

fn de_serialize_from_slice(slice: &[u8]) -> ServerCmdAddPeopleToAddressBook {
    serde_cbor::from_slice(slice).unwrap()
}

fn create_entity() -> ServerCmdAddPeopleToAddressBook<'static> {
    ServerCmdAddPeopleToAddressBook {
        person_list: Vec::from([
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
            },
        ]),
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
struct ServerCmdAddPeopleToAddressBook<'a> {
    #[serde(borrow = "'a")]
    person_list: Vec<Person<'a>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
struct Person<'a> {
    #[serde(borrow = "'a")]
    first_name: BooStr<'a>,
    #[serde(borrow = "'a")]
    last_name: BooStr<'a>,
}
