use std::alloc::System;

use serde::{Deserialize, Serialize};
use stats_alloc::{StatsAlloc, INSTRUMENTED_SYSTEM};

use abin::{AnyBin, Bin, Str};
use utils::*;

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

pub mod utils;

/// Demonstrates how to use serde using strings;
#[test]
fn serialize_deserialize() {
    deserialize_serialize_small();
    deserialize_serialize_large();
}

/// De-serialization / serialization with small strings that can be stack-allocated
/// (no allocation).
fn deserialize_serialize_small() {
    let short = "short";

    // note: no allocation here
    let entity = mem_scoped(
        &GLOBAL,
        &MaAnd(&[
            &MaExactNumberOfAllocations(0),
            &MaExactNumberOfReAllocations(0),
            &MaExactNumberOfDeAllocations(0),
        ]),
        || {
            Entity {
                id: 45,
                // here we create a short string (stack-allocated)
                string_a: short.into(),
                // empty: so stack allocated
                string_b: Str::from_static(""),
            }
        },
    );

    let as_vec = serde_cbor::to_vec(&entity).unwrap();

    // note: no allocation here
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
            assert_eq!(entity, restored);
        },
    );
}

/// De-serialization / serialization with large binaries (allocation required).
fn deserialize_serialize_large() {
    // note: no allocation here
    let entity = mem_scoped(
        &GLOBAL,
        &MaAnd(&[
            &MaExactNumberOfAllocations(0),
            &MaExactNumberOfReAllocations(0),
            &MaExactNumberOfDeAllocations(0),
        ]),
        || Entity {
            id: 45,
            string_a: Str::from_static(
                "This is somewhat longer; this will not fit \
                on stack - longer - even longer.",
            ),
            string_b: Str::from_static(
                "Longer and longer and longer and longer and \
                even longer... again, even longer. Longer and longer.",
            ),
        },
    );

    let as_vec = serde_cbor::to_vec(&entity).unwrap();

    // note: 2 allocations (& de-allocations) here.
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
            assert_eq!(entity, restored);
        },
    );
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Clone, Debug)]
pub struct Entity {
    pub id: u64,
    pub string_a: Str,
    pub string_b: Str,
}
