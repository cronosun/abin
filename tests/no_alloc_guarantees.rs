use std::alloc::System;

use stats_alloc::{INSTRUMENTED_SYSTEM, StatsAlloc};

use abin::{AnyBin, AnyRc, ArcBin, EmptyBin, IntoSync, IntoUnSync, IntoUnSyncView, NoVecCapShrink, RcBin, StackBin, StaticBin, VecBin};
use utils::*;

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

pub mod utils;

/// This tests some guarantees that are given that do not heap-allocate.
#[test]
fn no_alloc_guarantees() {
    empty();
    small_binaries_are_stack_allocated();
    vec_bin_create_and_into_vec();
    convert_into_sync_un_sync();
    no_alloc_clone();
    slice_does_not_allocate();
    into_vec_does_not_allocate();
    rc_from_vec();
}

/// The empty binary is stored on the stack; so no allocation here.
fn empty() {
    mem_scoped(&GLOBAL, &MaNoAllocNoDealloc, || {
        EmptyBin::new();
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
        RcBin::from_vec(empty_vec_1);
        RcBin::copy_from_slice(empty_slice);
        ArcBin::from_vec(empty_vec_2);
        ArcBin::copy_from_slice(empty_slice);

        RcBin::copy_from_slice(max_stack_alloc_vec_1.as_slice());
        RcBin::from_vec(max_stack_alloc_vec_1);
        ArcBin::copy_from_slice(max_stack_alloc_vec_2.as_slice());
        ArcBin::from_vec(max_stack_alloc_vec_2);
    });
}

/// VecBin just wraps a vec and returns the same vec when `into_vec` is called (so no
/// allocation in that case).
fn vec_bin_create_and_into_vec() {
    let bin_gen = BinGen::new(0, 200);
    let vec = bin_gen.generate_to_vec();
    // this of course needs to allocate, that's why it's outside 'mem_scoped'.
    let vec_bin_copy_from_slice = VecBin::copy_from_slice(vec.as_slice(), false);

    mem_scoped(&GLOBAL, &MaNoAllocNoReAlloc, || {
        // no allocation
        let vec_bin = VecBin::from_vec(vec, false);
        // and get the vector back
        let extracted_vec = vec_bin.into_vec();
        // also this does not allocate
        let extracted_vec_from_other = vec_bin_copy_from_slice.into_vec();
        assert_eq!(extracted_vec, extracted_vec_from_other);
    });
}

/// `into_sync` and `un_sync_convert` are usually allocation-free (there's an exception when
/// there are multiple references to reference-counted binaries). `un_sync` is always
/// allocation-free.
fn convert_into_sync_un_sync() {
    let bin_gen = BinGen::new(0, 200);
    let vec = bin_gen.generate_to_vec();

    let bin_1 = EmptyBin::new().un_sync();
    let bin_2 = StaticBin::from("Hello, slice!".as_bytes()).un_sync();
    let bin_3 = RcBin::from_vec(vec.clone());
    let bin_4 = ArcBin::from_vec(vec.clone()).un_sync();
    let bin_5 = RcBin::from_vec(generate_small_vec_that_fits_on_stack());
    let bin_6 = ArcBin::from_vec(generate_small_vec_that_fits_on_stack()).un_sync();
    let bin_7 = VecBin::from_vec(vec, false).un_sync();

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

/// Cloning is usually allocation-free. Exception: `VecBin`.
fn no_alloc_clone() {
    let bin_gen = BinGen::new(0, 200);
    let vec = bin_gen.generate_to_vec();

    let bin_1 = EmptyBin::new().un_sync();
    let bin_2 = StaticBin::from("Hello, slice!".as_bytes()).un_sync();
    let bin_3 = RcBin::from_vec(vec.clone());
    let bin_4 = ArcBin::from_vec(vec.clone()).un_sync();
    let bin_5 = RcBin::from_vec(generate_small_vec_that_fits_on_stack());
    let bin_6 = ArcBin::from_vec(generate_small_vec_that_fits_on_stack()).un_sync();
    let bin_7_allocates = VecBin::from_vec(vec, false).un_sync();

    mem_scoped(&GLOBAL, &MaNoAllocNoReAlloc, || {
        // empty bin is stack-only; so never allocates.
        let _ignored = bin_1.clone();
        let _ignored = bin_2.clone();
        // reference-counted binaries never allocate.
        let _ignored = bin_3.clone();
        // reference-counted binaries never allocate.
        let _ignored = bin_4.clone();
        let _ignored = bin_5.clone();
        let _ignored = bin_6.clone();
    });

    mem_scoped(&GLOBAL, &MaDoesAllocate, || {
        // THIS DOES ALLOCATE
        let _ignored = bin_7_allocates.clone();
    });
}

/// Slicing is usually allocation-free. Exception: `VecBin`.
fn slice_does_not_allocate() {
    let len = 200;
    let bin_gen = BinGen::new(0, len);
    let vec = bin_gen.generate_to_vec();

    let bin_1 = EmptyBin::new().un_sync();
    let bin_2 = StaticBin::from("Hello, slice!".as_bytes()).un_sync();
    let bin_3 = RcBin::from_vec(vec.clone());
    let bin_4 = ArcBin::from_vec(vec.clone()).un_sync();
    let bin_5 = RcBin::from_vec(generate_small_vec_that_fits_on_stack());
    let bin_6 = ArcBin::from_vec(generate_small_vec_that_fits_on_stack()).un_sync();
    let bin_7_allocates = VecBin::from_vec(vec, false).un_sync();

    mem_scoped(&GLOBAL, &MaNoAllocNoReAlloc, || {
        // empty bin is stack-only; so never allocates.
        assert_eq!(&[] as &[u8], bin_1.slice(0..0).unwrap().as_slice());
        assert_eq!("ello".as_bytes(), bin_2.slice(1..5).unwrap().as_slice());
        assert!(bin_3.slice(10..len - 5).is_some());
        assert!(bin_4.slice(10..len - 5).is_some());
        assert!(bin_5.slice(1..StackBin::max_len() - 1).is_some());
        assert!(bin_6.slice(1..StackBin::max_len() - 1).is_some());
    });

    mem_scoped(&GLOBAL, &MaDoesAllocate, || {
        // THIS DOES ALLOCATE
        assert!(bin_7_allocates.slice(10..len - 5).is_some());
    });
}

/// `into_vec` is allocation-free for `EmptyBin`, `VecBin` and for reference-counted binaries
/// with only one reference.
fn into_vec_does_not_allocate() {
    let len = 200;
    let bin_gen = BinGen::new(0, len);
    let vec = bin_gen.generate_to_vec();

    let bin_1 = EmptyBin::new().un_sync();
    let bin_2_allocates = StaticBin::from("Hello, slice!".as_bytes()).un_sync();
    let bin_3 = RcBin::from_vec(vec.clone());
    let bin_4 = ArcBin::from_vec(vec.clone()).un_sync();
    let bin_5_allocates = RcBin::from_vec(generate_small_vec_that_fits_on_stack());
    let bin_6_allocates = ArcBin::from_vec(generate_small_vec_that_fits_on_stack()).un_sync();
    let bin_7 = VecBin::from_vec(vec, false).un_sync();

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

/// Creating a rc is alloc free under some conditions:
///
///   * Enough capacity (see `AnyRc::overhead_bytes()`)
///   * Do not use a capacity shrinker (or use a vec that does not have too much excess).
fn rc_from_vec() {
    let generator = BinGen::new(0, 1024 * 32);

    let vec_for_sync = generator.generate_to_vec_shrink(ArcBin::overhead_bytes());
    let vec_for_non_sync = generator.generate_to_vec_shrink(RcBin::overhead_bytes());

    // reserve additional 64k (excess)
    let mut vec_for_sync_much_excess = generator.generate_to_vec();
    vec_for_sync_much_excess.reserve(1024 * 64);
    let mut vec_for_non_sync_much_excess = generator.generate_to_vec();
    vec_for_non_sync_much_excess.reserve(1024 * 64);

    let (_, _, _, _) = mem_scoped(&GLOBAL, &MaNoAllocNoReAlloc, || {
        let bin1 = ArcBin::from_vec(vec_for_sync);
        let bin2 = RcBin::from_vec(vec_for_non_sync);

        // can also construct from a vec with much excess (but in this case, we have to choose a
        // different shrinker).
        let bin3 = ArcBin::from_with_cap_shrink::<NoVecCapShrink>(vec_for_sync_much_excess);
        let bin4 = ArcBin::from_with_cap_shrink::<NoVecCapShrink>(vec_for_non_sync_much_excess);

        (bin1, bin2, bin3, bin4)
    });
}

/// See `StackBin::max_len()`: That's the maximum that fits on stack.
fn generate_small_vec_that_fits_on_stack() -> Vec<u8> {
    let bin_gen = BinGen::new(0, StackBin::max_len());
    bin_gen.generate_to_vec()
}
