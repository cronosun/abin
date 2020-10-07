#![cfg(feature = "serde")]

use std::alloc::System;

use serde::{Deserialize, Serialize};
use stats_alloc::{StatsAlloc, INSTRUMENTED_SYSTEM};

use abin::{AnyBin, BinFactory, DefaultScopes, NewSBin, NewSStr, SBin, SStr, StrFactory};
use utils::*;

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

pub mod utils;

/// This demonstrates that it's possible to have a zero-allocation de-serialization. It's
/// a client-server example.
///
/// The important thing here is (see `ServerRequest`):
///
/// * `#[serde(deserialize_with = "abin::ri_deserialize_sync_str")]`
/// * `#[serde(deserialize_with = "abin::ri_deserialize_sync_bin")]`
#[test]
fn zero_allocation_deserialization() {
    // no memory-leak is allowed of course
    mem_scoped(&GLOBAL, &MaNoLeak, || {
        // serialization (this allocates) -> this happens on the client-side
        let sbin = {
            let request = create_server_request();
            // serialize
            let vec = serde_cbor::to_vec(&request).unwrap();
            NewSBin::from_given_vec(vec)
        };

        // this is the de-serialization (it does not allocate).
        mem_scoped(&GLOBAL, &MaNoAllocNoReAlloc, || {
            server_process_message(sbin);
        });
    });
}

/// The server gets the message from the client/from network as `SBin` (converted from Vec<u8>).
fn server_process_message(msg: SBin) {
    // create a scope for re-integration
    let scope_setup = DefaultScopes::sync(&msg);
    // de-serialize
    let request =
        scope_setup.scoped(|| serde_cbor::from_slice::<ServerRequest>(msg.as_slice()).unwrap());

    let (bin1, bin2) = (request.huge_binary_1, request.huge_binary_2);

    // the request results in two database-commands
    let db_command1 = DatabaseCommand {
        command_type: 0,
        huge_binary: bin1,
    };
    let db_command2 = DatabaseCommand {
        command_type: 0,
        huge_binary: bin2,
    };

    // let the database process those two commands.
    database_process_message(db_command1);
    database_process_message(db_command2);
}

fn database_process_message(command: DatabaseCommand) {
    match command.command_type {
        0 => {
            // do something
            let _binary = command.huge_binary;
        }
        _ => {
            // do something else
            let _binary = command.huge_binary;
        }
    }
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Clone, Debug)]
pub struct ServerRequest {
    pub request_id: u64,
    #[serde(deserialize_with = "abin::ri_deserialize_sstr")]
    pub user_name: SStr,
    #[serde(deserialize_with = "abin::ri_deserialize_sbin")]
    pub huge_binary_1: SBin,
    #[serde(deserialize_with = "abin::ri_deserialize_sbin")]
    pub huge_binary_2: SBin,
}

pub struct DatabaseCommand {
    pub command_type: u64,
    pub huge_binary: SBin,
}

fn create_server_request() -> ServerRequest {
    ServerRequest {
        request_id: 25,
        user_name: NewSStr::from_static(
            "a_very_long_user_name_that_does_not_fit_on_stack@my_long_server.com \
            ['The user also has a readable name - this name is long too']",
        ),
        huge_binary_1: NewSBin::from_given_vec(BinGen::new(0, 1024 * 32).generate_to_vec_shrink(0)),
        huge_binary_2: NewSBin::from_given_vec(BinGen::new(0, 1024 * 16).generate_to_vec_shrink(0)),
    }
}
