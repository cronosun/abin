use std::alloc::System;

use stats_alloc::{StatsAlloc, INSTRUMENTED_SYSTEM};

use abin::{AnyBin, Factory, IntoSync, IntoUnSync, IntoUnSyncView, NeverShrink, New, SNew};
use utils::*;

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

pub mod utils;

const STACK_BIN_LEN: usize = 3;

/// This tests some guarantees that are given that do not heap-allocate.
#[test]
fn no_alloc_guarantees() {
    empty();
    small_binaries_are_stack_allocated();
    vec_can_be_extracted_without_allocation();
    convert_into_sync_un_sync();
    no_alloc_clone();
    slice_does_not_allocate();
    into_vec_does_not_allocate();
    // TODO rc_from_vec();
}

/// The empty binary is stored on the stack; so no allocation here.
fn empty() {
    mem_scoped(&GLOBAL, &MaNoAllocNoDealloc, || {
        New::empty();
        SNew::empty();
    });
}

/// Small binaries are stored on the stack (up to `StackBin::max_len()` bytes).
fn small_binaries_are_stack_allocated() {
    let empty_slice: &[u8] = &[];
    let empty_vec_1 = Vec::from(empty_slice);
    let empty_vec_2 = Vec::from(empty_slice);

    // 'StackBin::max_len()': maximum that can be stored on stack.
    let max_stack_alloc_vec_1 = generate_small_vec_that_fits_on_stack();
    let max_stack_alloc_vec_2 = generate_small_vec_that_fits_on_stack();

    mem_scoped(&GLOBAL, &MaNoAllocNoReAlloc, || {
        New::from_given_vec(empty_vec_1);
        New::copy_from_slice(empty_slice);
        SNew::from_given_vec(empty_vec_2);
        SNew::copy_from_slice(empty_slice);

        New::copy_from_slice(max_stack_alloc_vec_1.as_slice());
        New::from_given_vec(max_stack_alloc_vec_1);
        SNew::copy_from_slice(max_stack_alloc_vec_2.as_slice());
        SNew::from_given_vec(max_stack_alloc_vec_2);
    });
}

/// As long as there are not multiple references pointing to the binary, a vec can always be
/// extracted without allocation.
fn vec_can_be_extracted_without_allocation() {
    let bin_gen = BinGen::new(0, 800);
    let vec = bin_gen.generate_to_vec();
    let vec_clone = vec.clone();
    let small_vec: Vec<u8> = Vec::from(&[4u8, 8u8] as &[u8]);
    let small_vec_clone = small_vec.clone();

    mem_scoped(&GLOBAL, &MaNoAllocNoReAlloc, || {
        let large = New::from_given_vec(vec);
        let small = New::from_given_vec(small_vec);

        // and get the vector back
        let large_vec = large.into_vec();
        let small_vec = small.into_vec();

        assert_eq!(vec_clone, large_vec);
        assert_eq!(small_vec_clone, small_vec);
    });
}

/// `into_sync` and `un_sync_convert` are usually allocation-free (there's an exception when
/// there are multiple references to reference-counted binaries). `un_sync` is always
/// allocation-free.
fn convert_into_sync_un_sync() {
    let bin_gen = BinGen::new(0, 200);
    let vec = bin_gen.generate_to_vec();

    let bin_1 = New::empty();
    let bin_2 = New::from_static("Hello, slice!".as_bytes());
    let bin_3 = New::copy_from_slice(vec.as_slice());
    let bin_4 = SNew::copy_from_slice(vec.as_slice()).un_sync();
    let bin_5 = New::from_given_vec(generate_small_vec_that_fits_on_stack());
    let bin_6 = SNew::from_given_vec(generate_small_vec_that_fits_on_stack()).un_sync();
    let bin_7 = SNew::from_given_vec(vec).un_sync();

    mem_scoped(&GLOBAL, &MaNoAllocNoReAlloc, || {
        // convert to sync
        let sync_bin1 = bin_1.into_sync();
        let sync_bin2 = bin_2.into_sync();
        let sync_bin3 = bin_3.into_sync();
        // note: for bin4 & 5: this would allocate if there were more shared references pointing to them.
        let sync_bin4 = bin_4.into_sync();
        let sync_bin5 = bin_5.into_sync();
        let sync_bin6 = bin_6.into_sync();
        let sync_bin7 = bin_7.into_sync();

        // convert back to un-sync
        sync_bin1.un_sync_convert();
        sync_bin2.un_sync_convert();
        sync_bin3.un_sync_convert();
        // note: for bin4 & 5: this would allocate if there were more shared references pointing to them.
        sync_bin4.un_sync_convert();
        sync_bin5.un_sync_convert();
        sync_bin6.un_sync_convert();
        sync_bin7.un_sync_convert();
    });
}

/// Cloning is usually allocation-free.
fn no_alloc_clone() {
    let bin_gen = BinGen::new(0, 200);
    let vec = bin_gen.generate_to_vec();

    let bin_1 = New::empty();
    let bin_2 = New::from_static("Hello, slice!".as_bytes());
    let bin_3 = New::from_given_vec(vec.clone());
    let bin_4 = SNew::from_given_vec(vec.clone()).un_sync();
    let bin_5 = New::from_given_vec(generate_small_vec_that_fits_on_stack());
    let bin_6 = SNew::from_given_vec(generate_small_vec_that_fits_on_stack()).un_sync();

    mem_scoped(&GLOBAL, &MaNoAllocNoReAlloc, || {
        let _ignored = bin_1.clone();
        let _ignored = bin_2.clone();
        let _ignored = bin_3.clone();
        let _ignored = bin_4.clone();
        let _ignored = bin_5.clone();
        let _ignored = bin_6.clone();
    });
}

/// Slicing is usually allocation-free.
fn slice_does_not_allocate() {
    let len = 200;
    let bin_gen = BinGen::new(0, len);
    let vec = bin_gen.generate_to_vec();

    let bin_1 = New::empty();
    let bin_2 = New::from_static("Hello, slice!".as_bytes());
    let bin_3 = New::from_given_vec(vec.clone());
    let bin_4 = SNew::from_given_vec(vec.clone()).un_sync();
    let bin_5 = New::from_given_vec(generate_small_vec_that_fits_on_stack());
    let bin_6 = SNew::from_given_vec(generate_small_vec_that_fits_on_stack()).un_sync();

    mem_scoped(&GLOBAL, &MaNoAllocNoReAlloc, || {
        // empty bin is stack-only; so never allocates.
        assert_eq!(&[] as &[u8], bin_1.slice(0..0).unwrap().as_slice());
        assert_eq!("ello".as_bytes(), bin_2.slice(1..5).unwrap().as_slice());
        assert!(bin_3.slice(10..len - 5).is_some());
        assert!(bin_4.slice(10..len - 5).is_some());
        assert!(bin_5.slice(1..STACK_BIN_LEN - 1).is_some());
        assert!(bin_6.slice(1..STACK_BIN_LEN - 1).is_some());
    });
}

/// `into_vec` is allocation-free for `EmptyBin`, ... and for reference-counted binaries
/// with only one reference.
fn into_vec_does_not_allocate() {
    let len = 200;
    let bin_gen = BinGen::new(0, len);
    let vec = bin_gen.generate_to_vec();

    let bin_1 = New::empty();
    let bin_2_allocates = New::from_static("Hello, slice!".as_bytes());
    let bin_3 = New::copy_from_slice(vec.as_slice());
    let bin_4 = SNew::copy_from_slice(vec.as_slice()).un_sync();
    let bin_5_allocates = New::from_given_vec(generate_small_vec_that_fits_on_stack());
    let bin_6_allocates = SNew::from_given_vec(generate_small_vec_that_fits_on_stack()).un_sync();
    let bin_7 = New::from_given_vec(vec);

    mem_scoped(&GLOBAL, &MaNoAllocNoReAlloc, || {
        // returns an empty vec
        assert_eq!(bin_1.into_vec(), Vec::new());
        // reference-counted binaries do not allocate unless there are multiple references pointing to them.
        bin_3.into_vec();
        // reference-counted binaries do not allocate unless there are multiple references pointing to them.
        bin_4.into_vec();
        bin_7.into_vec();
    });

    mem_scoped(&GLOBAL, &MaDoesAllocate, || {
        // THIS DOES ALLOCATE
        // it's static, so we have to allocate.
        bin_2_allocates.into_vec();
    });
    mem_scoped(&GLOBAL, &MaDoesAllocate, || {
        // THIS DOES ALLOCATE
        // it's saved on the stack, so we have to allocate.
        bin_5_allocates.into_vec();
    });
    mem_scoped(&GLOBAL, &MaDoesAllocate, || {
        // THIS DOES ALLOCATE
        // it's saved on the stack, so we have to allocate.
        bin_6_allocates.into_vec();
    });
}

/// Creating a bin is alloc free under some conditions:
///
///   * Given vec has enough capacity (see `AnyRc::overhead_bytes()`)
///   * Do not use a capacity shrinker (or use a vec that does not have too much excess).
/*fn rc_from_vec() {
    let generator = BinGen::new(0, 1024 * 32);

    let vec_for_sync = generator.generate_to_vec_shrink(New::vec_excess());
    let vec_for_non_sync = generator.generate_to_vec_shrink(SNew::vec_excess());

    // reserve additional 64k (excess)
    let mut vec_for_sync_much_excess = generator.generate_to_vec();
    vec_for_sync_much_excess.reserve(1024 * 64);
    let mut vec_for_non_sync_much_excess = generator.generate_to_vec();
    vec_for_non_sync_much_excess.reserve(1024 * 64);

    let (_, _, _, _) = mem_scoped(&GLOBAL, &MaNoAllocNoReAlloc, || {
        let bin1 = SNew::from_given_vec(vec_for_sync);
        let bin2 = New::from_given_vec(vec_for_non_sync);

        // can also construct from a vec with much excess (but in this case, we have to choose a
        // different shrinker).
        let bin3 = SNew::from_given_vec_with_config::<NeverShrink>(vec_for_sync_much_excess);
        let bin4 = SNew::from_given_vec_with_config::<NeverShrink>(vec_for_non_sync_much_excess);

        (bin1, bin2, bin3, bin4)
    });
}*/

/// See `StackBin::max_len()`: That's the maximum that fits on stack.
fn generate_small_vec_that_fits_on_stack() -> Vec<u8> {
    let bin_gen = BinGen::new(0, STACK_BIN_LEN);
    bin_gen.generate_to_vec()
}
