use std::alloc::System;

use stats_alloc::{StatsAlloc, INSTRUMENTED_SYSTEM};

use abin::{AnyBin, BinFactory, BinSegment, NewBin, NewSBin, SegmentIterator, SegmentsSlice};
use utils::*;

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

pub mod utils;

const STACK_MAX_LEN: usize = 3 * core::mem::size_of::<usize>() - 1;

/// Tests that from_iter and copy_from_slice only allocate once (e.g. they create a vector
/// that has enough capacity to contain the meta-data).
#[test]
fn single_allocation() {
    mem_scoped(&GLOBAL, &MaNoLeak, || {
        // note: Start must be `STACK_MAX_LEN + 1)` (if not, we'd have no allocation).
        for index in (STACK_MAX_LEN + 1)..255 {
            let vec = BinGen::new(index as u8, index as usize).generate_to_vec();
            copy_from_slice::<NewBin>(vec.as_slice());
            from_iter::<NewBin>(vec.as_slice());
            copy_from_slice::<NewSBin>(vec.as_slice());
            from_iter::<NewSBin>(vec.as_slice());
        }

        from_multiple_parts::<NewBin>();
        from_multiple_parts::<NewSBin>();
    });
}

fn copy_from_slice<T: BinFactory>(slice: &[u8]) {
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

fn from_iter<T: BinFactory>(slice: &[u8]) {
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
fn from_multiple_parts<T: BinFactory>() {
    // create multiple binaries
    let item_1 = T::empty();
    let item_2 = "This is slice one; a bit too large for the stack.".as_bytes();
    let item_3 = "Another slice. a bit too large for the stack.".as_bytes();
    let item_4 = T::empty();
    let item_5 = T::from_static("This is a static binary".as_bytes());
    let item_6 = T::from_given_vec(BinGen::new(0, 80).generate_to_vec());
    let item_7 = T::from_given_vec(BinGen::new(0, 90).generate_to_vec());
    let item_8 = T::from_given_vec(BinGen::new(0, 100).generate_to_vec());
    let item_9 = T::empty();

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
    let expected_len = expected_result.len();

    // one single allocation / no re-allocations
    let bin = mem_scoped(
        &GLOBAL,
        &MaAnd(&[
            &MaExactNumberOfAllocations(1),
            &MaExactNumberOfReAllocations(0),
        ]),
        || {
            // it's possible to chain all those binaries with just one single allocation
            let items: &mut [BinSegment<T::T>] = &mut [
                BinSegment::Bin(item_1),
                item_2.into(),
                item_3.into(),
                BinSegment::Bin(item_4),
                BinSegment::Bin(item_5),
                BinSegment::Bin(item_6),
                BinSegment::Bin(item_7),
                BinSegment::Bin(item_8),
                BinSegment::Bin(item_9),
            ];
            let iterator = SegmentsSlice::new(items);

            // the iterator guarantees to return the exact 'exact_number_of_bytes'
            assert_eq!(expected_len, iterator.exact_number_of_bytes().unwrap());

            // one single allocation here
            let bin = T::from_segments(iterator);
            bin
        },
    );

    assert_eq!(bin.as_slice(), expected_result.as_slice());
}
