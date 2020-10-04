use std::alloc::System;

use serde::{Deserialize, Serialize};
use stats_alloc::{StatsAlloc, INSTRUMENTED_SYSTEM};

use abin::{AnyBin, Bin, BinFactory, NewBin};
use utils::*;

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

pub mod utils;

/// Conservative length (to make sure this fits on the stack on every platform).
const STACK_BIN_LEN: usize = 3;

/// Demonstrates how to use serde;
#[test]
fn serialize_deserialize() {
    deserialize_serialize_small();
    deserialize_serialize_large();
}

/// De-serialization / serialization with small binaries that can be stack-allocated
/// (no allocation).
fn deserialize_serialize_small() {
    let item_vec = BinGen::new(0, STACK_BIN_LEN).generate_to_vec();
    let item_2_vec = NewBin::empty();

    let original = Entity {
        id: 45,
        item: NewBin::copy_from_slice(item_vec.as_slice()),
        item_2: NewBin::copy_from_slice(item_2_vec.as_slice()),
    };

    let as_vec = serde_cbor::to_vec(&original).unwrap();

    // note: This should not allocate, since all binaries should be stack-allocated
    mem_scoped(
        &GLOBAL,
        &MaAnd(&[
            &MaExactNumberOfAllocations(0),
            &MaExactNumberOfReAllocations(0),
            &MaExactNumberOfDeAllocations(0),
        ]),
        || {
            let restored: Entity = serde_cbor::from_slice(as_vec.as_slice()).unwrap();
            // must be equal
            assert_eq!(original, restored);
        },
    );
}

/// De-serialization / serialization with large binaries (allocation required).
fn deserialize_serialize_large() {
    let item_vec = BinGen::new(0, 1024).generate_to_vec();
    let item_2_vec = BinGen::new(0, 2048).generate_to_vec();

    let original = Entity {
        id: 55,
        item: NewBin::copy_from_slice(item_vec.as_slice()),
        item_2: NewBin::copy_from_slice(item_2_vec.as_slice()),
    };

    let as_vec = serde_cbor::to_vec(&original).unwrap();

    // yes, this will allocate; 2 allocations (one for each binary; and then 2 de-allocations).
    mem_scoped(
        &GLOBAL,
        &MaAnd(&[
            &MaExactNumberOfAllocations(2),
            &MaExactNumberOfReAllocations(0),
            &MaExactNumberOfDeAllocations(2),
        ]),
        || {
            let restored: Entity = serde_cbor::from_slice(as_vec.as_slice()).unwrap();
            // must be equal
            assert_eq!(original, restored);
        },
    );
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Clone, Debug)]
pub struct Entity {
    pub id: u64,
    pub item: Bin,
    pub item_2: Bin,
}
