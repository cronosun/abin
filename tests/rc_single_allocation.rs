use std::alloc::System;

use stats_alloc::{StatsAlloc, INSTRUMENTED_SYSTEM};

use abin::{AnyBin, AnyRc, ArcBin, Bin, RcBin, StackBin, SyncBin};
use utils::*;

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

pub mod utils;

/// Tests that rc from_iter and copy_from_slice only allocate once (e.g. they create a vector
/// that has enough capacity to contain the meta-data).
#[test]
fn rc_single_allocation() {
    mem_scoped(&GLOBAL, &MaNoLeak, || {
        // note: Start must be `(StackBin::max_len() + 1)` (if not, we'd have no allocation).
        for index in (StackBin::max_len() + 1)..255 {
            let vec = BinGen::new(index as u8, index as usize).generate_to_vec();
            copy_from_slice::<RcBin, Bin>(vec.as_slice());
            from_iter::<RcBin, Bin>(vec.as_slice());
            copy_from_slice::<ArcBin, SyncBin>(vec.as_slice());
            from_iter::<ArcBin, SyncBin>(vec.as_slice());
        }
    });
}

fn copy_from_slice<T: AnyRc<T = TBin>, TBin: AnyBin>(slice: &[u8]) {
    mem_scoped(
        &GLOBAL,
        &MaAnd(&[
            &MaExactNumberOfAllocations(1),
            &MaExactNumberOfDeAllocations(1),
        ]),
        || {
            // one single allocation here
            let bin = T::copy_from_slice(slice);
            assert_eq!(bin.as_slice(), slice);
            // and one single de-allocation for `bin.drop()`
        },
    );
}

fn from_iter<T: AnyRc<T = TBin>, TBin: AnyBin>(slice: &[u8]) {
    mem_scoped(
        &GLOBAL,
        &MaAnd(&[
            &MaExactNumberOfAllocations(1),
            &MaExactNumberOfDeAllocations(1),
        ]),
        || {
            // one single allocation
            let bin = T::from_iter(slice.iter().map(|item| *item));
            assert_eq!(bin.as_slice(), slice);
            // and one single de-allocation for `bin.drop()`
        },
    );
}
