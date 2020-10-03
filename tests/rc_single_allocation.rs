use std::alloc::System;

use stats_alloc::{StatsAlloc, INSTRUMENTED_SYSTEM};

use abin::{
    AnyBin, AnyRc, ArcBin, Bin, ChainSlicesIter, EmptyBin, RcBin, StackBin, StaticBin, SBin,
    VecBin,
};
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
            copy_from_slice::<ArcBin, SBin>(vec.as_slice());
            from_iter::<ArcBin, SBin>(vec.as_slice());
        }

        rc_from_multiple_parts::<RcBin, Bin>();
        rc_from_multiple_parts::<ArcBin, SBin>();
    });
}

fn copy_from_slice<T: AnyRc<T = TBin>, TBin: AnyBin>(slice: &[u8]) {
    mem_scoped(
        &GLOBAL,
        &MaAnd(&[
            &MaExactNumberOfAllocations(1),
            &MaExactNumberOfDeAllocations(1),
            &MaExactNumberOfReAllocations(0),
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
            &MaExactNumberOfReAllocations(0),
        ]),
        || {
            // one single allocation
            let bin = T::from_iter(slice.iter().map(|item| *item));
            assert_eq!(bin.as_slice(), slice);
            // and one single de-allocation for `bin.drop()`
        },
    );
}

/// it's also possible to collect multiple binaries and slices and still have one single allocation.
fn rc_from_multiple_parts<T: AnyRc<T = TBin>, TBin: AnyBin>() {
    // create multiple binaries
    let item_1 = EmptyBin::new();
    let item_2 = "This is slice one; a bit too large for the stack.".as_bytes();
    let item_3 = "Another slice. a bit too large for the stack.".as_bytes();
    let item_4 = EmptyBin::new();
    let item_5 = StaticBin::from("This is a static binary".as_bytes());
    let item_6 = VecBin::from_vec(BinGen::new(0, 80).generate_to_vec(), false);
    let item_7 = RcBin::from_vec(BinGen::new(0, 90).generate_to_vec());
    let item_8 = ArcBin::from_vec(BinGen::new(0, 100).generate_to_vec());
    let item_9 = EmptyBin::new();

    let expected_len = item_1.len()
        + item_2.len()
        + item_3.len()
        + item_4.len()
        + item_5.len()
        + item_6.len()
        + item_7.len()
        + item_8.len()
        + item_9.len();

    // one single allocation
    let bin = mem_scoped(
        &GLOBAL,
        &MaAnd(&[
            &MaExactNumberOfAllocations(1),
            &MaExactNumberOfDeAllocations(0),
            &MaExactNumberOfReAllocations(0),
        ]),
        || {
            // it's possible to chain all those binaries into a Rc with just one single allocation
            let items = &[
                item_1.as_slice(),
                item_2,
                item_3,
                item_4.as_slice(),
                item_5.as_slice(),
                item_6.as_slice(),
                item_7.as_slice(),
                item_8.as_slice(),
                item_9.as_slice(),
            ] as &[&[u8]];
            let iterator = ChainSlicesIter::from(items);

            // the iterator guarantees to return the exact len / size_hint
            assert_eq!(expected_len, iterator.len());
            assert_eq!((expected_len, Some(expected_len)), iterator.size_hint());

            // one single allocation here
            let bin = T::from_iter(iterator);
            bin
        },
    );

    // make sure we got the correct result.
    let items = &[
        item_1.as_slice(),
        item_2,
        item_3,
        item_4.as_slice(),
        item_5.as_slice(),
        item_6.as_slice(),
        item_7.as_slice(),
        item_8.as_slice(),
        item_9.as_slice(),
    ];
    let expected_result: Vec<u8> = items
        .iter()
        .map(|item| item.iter())
        .flatten()
        .map(|item| *item)
        .collect();

    assert_eq!(bin.as_slice(), expected_result.as_slice());
}
