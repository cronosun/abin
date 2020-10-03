use std::alloc::System;
use std::cmp::{Ordering, max};
use std::hash::{Hash, Hasher};
use std::ops::Range;

use stats_alloc::{StatsAlloc, INSTRUMENTED_SYSTEM};

use abin::{AnyBin, Bin, IntoUnSyncView, New, Factory, SNew};
use utils::*;

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

pub mod utils;

/// Tests whether a vec/slice and a bin behave the same.
#[test]
fn test_same_behaviour_basics() {
    // make sure there are no leaks
    mem_scoped(&GLOBAL, &MaNoLeak, || {
        let excess = max(New::vec_excess(), SNew::vec_excess());

        same_behaviour_basics_static(&[]);
        same_behaviour_basics_static(&[3]);
        same_behaviour_basics_static(&[4, 8]);
        same_behaviour_basics_static(&[4, 8, 4, 8, 78, 90, 0, 47]);

        // small
        for index in 0..1024 {
            let bin_gen = BinGen::new(index as u8, index as usize);
            let vec = bin_gen.generate_to_vec_shrink(excess);
            same_behaviour_non_static(vec);
        }

        // large
        for step in 1..15 {
            let index = step * 2007;
            let bin_gen = BinGen::new(index as u8, index as usize);
            let vec = bin_gen.generate_to_vec_shrink(excess);
            same_behaviour_non_static(vec);
        }
    });
}

fn same_behaviour_basics_static(original: &'static [u8]) {
    same_behaviour(original, New::from_static(original).un_sync());
}

fn same_behaviour_non_static(original: Vec<u8>) {
    let rc_bin = New::from_vec(original.clone());
    let arc_bin = SNew::from_vec(original.clone()).un_sync();

    same_behaviour(original.as_slice(), rc_bin);
    same_behaviour(original.as_slice(), arc_bin);
}

fn same_behaviour(original: &[u8], bin: Bin) {
    same_behaviour_basics(original, &bin);
    {
        let bin_cloned = bin.clone();
        same_behaviour_basics(original, &bin_cloned);
    }
    let bin_sliced_full = bin.slice(0..bin.len()).unwrap();
    same_behaviour_basics(original, &bin_sliced_full);
}

fn same_behaviour_basics(original: &[u8], bin: &Bin) {
    assert_eq!(original.len(), bin.len(), "Same length");
    assert_eq!(original, bin.as_slice(), "Slice equality");
    same_hash(original, bin);

    // slice: edge cases
    check_slice(original, &bin, 0..0);
    check_slice(original, &bin, 0..1);
    check_slice(original, &bin, 1..0);
    check_slice(original, &bin, 1..1);

    // slice: out of range
    check_slice(original, &bin, 0..(bin.len() + 1));
    check_slice(original, &bin, 0..core::usize::MAX);
    // just within range
    check_slice(original, &bin, 0..bin.len());

    let cloned_bin = bin.clone();
    assert_eq!(bin, &cloned_bin);
    assert_eq!(bin.partial_cmp(&cloned_bin), Some(Ordering::Equal));
    assert_eq!(bin.cmp(&cloned_bin), Ordering::Equal);
}

/// a binary produces the same hash as a slice.
fn same_hash(original: &[u8], bin: &Bin) {
    let mut hasher1 = std::collections::hash_map::DefaultHasher::default();
    original.hash(&mut hasher1);

    let mut hasher2 = std::collections::hash_map::DefaultHasher::default();
    bin.hash(&mut hasher2);

    assert_eq!(hasher1.finish(), hasher2.finish());
}

fn check_slice(original: &[u8], bin: &Bin, range: Range<usize>) {
    let bin_sliced = bin.slice(range.clone());

    let original_sliced = original.get(range);

    if let Some(bin_sliced) = bin_sliced {
        if let Some(original_sliced) = original_sliced {
            assert_eq!(bin_sliced.as_slice(), original_sliced);
            let vec = bin_sliced.into_vec();
            assert_eq!(vec.as_slice(), original_sliced);
        } else {
            panic!("Bin slice returned a result but slice slice did not!")
        }
    } else {
        assert!(original_sliced.is_none());
    }
}
