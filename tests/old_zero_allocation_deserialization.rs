use std::alloc::System;

use serde::{Deserialize, Serialize};
use stats_alloc::{StatsAlloc, INSTRUMENTED_SYSTEM};

use abin::{
    maybe_shrink, AnyBin, DefaultExcessShrink, DefaultScopes, Factory, NewSBin, NewSStr, SBin,
    SStr, StrFactory,
};
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
        let vec = {
            let request = create_server_request();
            // serialize
            let mut vec = serde_cbor::to_vec(&request).unwrap();
            // we need to 'tweak' the vec a bit to make sure there's no allocation and no re-allocation:
            //  - if the excess is too large we'd get a re-allocation.
            //  - if the excess is too small we'd get a allocation.
            vec.reserve(NewSBin::vec_excess());
            maybe_shrink::<DefaultExcessShrink>(&mut vec, NewSBin::vec_excess());
            vec
        };

        // here the server gets the request (Vec<u8>) from the client...
        // this is the de-serialization (it does not allocate).
        mem_scoped(&GLOBAL, &MaNoAllocNoReAlloc, || {
            server_process_message(vec);
        });
    });
}

/// The server gets the message from the client/from network as `Vec<u8>`.
fn server_process_message(msg: Vec<u8>) {
    // Convert that to arc-bin
    let msg_as_bin = NewSBin::from_given_vec(msg);

    // create a scope for re-integration
    let scope_setup = DefaultScopes::sync(&msg_as_bin);
    // de-serialize
    let request = scope_setup
        .scoped(|| serde_cbor::from_slice::<ServerRequest>(msg_as_bin.as_slice()).unwrap());

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
    #[serde(deserialize_with = "abin::ri_deserialize_sync_str")]
    pub user_name: SStr,
    #[serde(deserialize_with = "abin::ri_deserialize_sync_bin")]
    pub huge_binary_1: SBin,
    #[serde(deserialize_with = "abin::ri_deserialize_sync_bin")]
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
        huge_binary_1: NewSBin::from_given_vec(
            BinGen::new(0, 1024 * 32).generate_to_vec_shrink(NewSBin::vec_excess()),
        ),
        huge_binary_2: NewSBin::from_given_vec(
            BinGen::new(0, 1024 * 16).generate_to_vec_shrink(NewSBin::vec_excess()),
        ),
    }
}
