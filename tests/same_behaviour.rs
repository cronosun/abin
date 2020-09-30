use std::alloc::System;
use std::ops::Range;

use stats_alloc::{StatsAlloc, INSTRUMENTED_SYSTEM};

use abin::{AnyBin, AnyRc, ArcBin, Bin, RcBin, StaticBin, VecBin};
use utils::*;

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

pub mod utils;

/// Tests whether a vec and a bin behave the same.
#[test]
fn test_same_behaviour_basics() {
    // make sure there are no leaks
    mem_scoped(&GLOBAL, &MaNoLeak, || {
        same_behaviour_basics_static(&[]);
        same_behaviour_basics_static(&[3]);
        same_behaviour_basics_static(&[4, 8]);
        same_behaviour_basics_static(&[4, 8, 4, 8, 78, 90, 0, 47]);

        // small
        for index in 0..1024 {
            let bin_gen = BinGen::new(index as u8, index as usize);
            let vec = bin_gen.generate_to_vec_shrink(RcBin::overhead_bytes());
            same_behaviour_non_static(vec);
        }

        // large
        for step in 1..15 {
            let index = step * 2007;
            let bin_gen = BinGen::new(index as u8, index as usize);
            let vec = bin_gen.generate_to_vec_shrink(RcBin::overhead_bytes());
            same_behaviour_non_static(vec);
        }
    });
}

fn same_behaviour_basics_static(original: &'static [u8]) {
    same_behaviour(original, StaticBin::from(original).un_sync());
}

fn same_behaviour_non_static(original: Vec<u8>) {
    let vec_bin_1 = VecBin::from_vec(original.clone(), true).un_sync();
    let vec_bin_2 = VecBin::from_vec(original.clone(), false).un_sync();
    let rc_bin = RcBin::from_vec(original.clone());
    let arc_bin = ArcBin::from_vec(original.clone()).un_sync();

    same_behaviour(original.as_slice(), vec_bin_1);
    same_behaviour(original.as_slice(), vec_bin_2);
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

    // edge cases
    check_slice(original, &bin, 0..0);
    check_slice(original, &bin, 0..1);
    check_slice(original, &bin, 1..0);
    check_slice(original, &bin, 1..1);

    // out of range
    check_slice(original, &bin, 0..(bin.len() + 1));
    check_slice(original, &bin, 0..core::usize::MAX);
    // just within range
    check_slice(original, &bin, 0..bin.len());
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
