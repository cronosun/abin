use std::alloc::System;

use stats_alloc::{StatsAlloc, INSTRUMENTED_SYSTEM};

use abin::{AnyBin, BinFactory, IntoSync, IntoUnSync, IntoUnSyncView, NewBin, NewSBin};
use utils::*;

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

pub mod utils;

/// maximum number of bytes that fit onto the stack.
const STACK_BIN_LEN: usize = 3 * core::mem::size_of::<usize>() - 1;

/// This tests some guarantees that are given that do not heap-allocate.
#[test]
fn no_alloc_guarantees() {
    // and of course no leaks
    mem_scoped(&GLOBAL, &MaNoLeak, || {
        empty();
        small_binaries_are_stack_allocated();
        vec_can_be_extracted_without_allocation();
        convert_into_sync_un_sync();
        no_alloc_clone();
        slice_does_not_allocate();
        into_vec_does_not_allocate();
    });
}

/// The empty binary is stored on the stack; so no allocation here.
fn empty() {
    mem_scoped(&GLOBAL, &MaNoAllocNoDealloc, || {
        NewBin::empty();
        NewSBin::empty();
    });
}

/// Small binaries are stored on the stack (up to `STACK_BIN_LEN` bytes).
fn small_binaries_are_stack_allocated() {
    let empty_slice: &[u8] = &[];
    let empty_vec_1 = Vec::from(empty_slice);
    let empty_vec_2 = Vec::from(empty_slice);

    // 'StackBin::max_len()': maximum that can be stored on stack.
    let max_stack_alloc_vec_1 = generate_small_vec_that_fits_on_stack();
    let max_stack_alloc_vec_2 = generate_small_vec_that_fits_on_stack();

    mem_scoped(&GLOBAL, &MaNoAllocNoReAlloc, || {
        NewBin::from_given_vec(empty_vec_1);
        NewBin::copy_from_slice(empty_slice);
        NewSBin::from_given_vec(empty_vec_2);
        NewSBin::copy_from_slice(empty_slice);

        NewBin::copy_from_slice(max_stack_alloc_vec_1.as_slice());
        NewBin::from_given_vec(max_stack_alloc_vec_1);
        NewSBin::copy_from_slice(max_stack_alloc_vec_2.as_slice());
        NewSBin::from_given_vec(max_stack_alloc_vec_2);
    });
}

/// As long as there are not multiple references pointing to the binary, a vec can always be
/// extracted without allocation (as long as it's not too small and thus gets stack-allocated).
fn vec_can_be_extracted_without_allocation() {
    let bin_gen = BinGen::new(0, 800);
    let vec = bin_gen.generate_to_vec();
    let vec_clone = vec.clone();
    let bin_1 = NewBin::from_given_vec(vec);
    let vec_from_bin = bin_1.into_vec();

    mem_scoped(&GLOBAL, &MaNoAllocNoReAlloc, || {
        let bin_2 = NewBin::from_given_vec(vec_from_bin);
        // and get the vector back
        let vec_from_bin_2 = bin_2.into_vec();
        assert_eq!(vec_clone, vec_from_bin_2);
    });
}

/// `into_sync` and `un_sync_convert` are usually allocation-free (there's an exception when
/// there are multiple references to reference-counted binaries). `un_sync` is always
/// allocation-free.
fn convert_into_sync_un_sync() {
    let bin_gen = BinGen::new(0, 200);
    let vec = bin_gen.generate_to_vec();

    let bin_1 = NewBin::empty();
    let bin_2 = NewBin::from_static("Hello, slice!".as_bytes());
    let bin_3 = NewBin::copy_from_slice(vec.as_slice());
    let bin_4 = NewSBin::copy_from_slice(vec.as_slice()).un_sync();
    let bin_5 = NewBin::from_given_vec(generate_small_vec_that_fits_on_stack());
    let bin_6 = NewSBin::from_given_vec(generate_small_vec_that_fits_on_stack()).un_sync();

    mem_scoped(&GLOBAL, &MaNoAllocNoReAlloc, || {
        // convert to sync
        let sync_bin1 = bin_1.into_sync();
        let sync_bin2 = bin_2.into_sync();
        let sync_bin3 = bin_3.into_sync();
        // note: for bin4 & 5: this would allocate if there were more shared references pointing to them.
        let sync_bin4 = bin_4.into_sync();
        let sync_bin5 = bin_5.into_sync();
        let sync_bin6 = bin_6.into_sync();

        // convert back to un-sync
        sync_bin1.un_sync_convert();
        sync_bin2.un_sync_convert();
        sync_bin3.un_sync_convert();
        // note: for bin4 & 5: this would allocate if there were more shared references pointing to them.
        sync_bin4.un_sync_convert();
        sync_bin5.un_sync_convert();
        sync_bin6.un_sync_convert();
    });
}

/// Cloning is usually allocation-free.
fn no_alloc_clone() {
    let bin_gen = BinGen::new(0, 200);
    let vec = bin_gen.generate_to_vec();

    let bin_1 = NewBin::empty();
    let bin_2 = NewBin::from_static("Hello, slice!".as_bytes());
    let bin_3 = NewBin::from_given_vec(vec.clone());
    let bin_4 = NewSBin::from_given_vec(vec.clone()).un_sync();
    let bin_5 = NewBin::from_given_vec(generate_small_vec_that_fits_on_stack());
    let bin_6 = NewSBin::from_given_vec(generate_small_vec_that_fits_on_stack()).un_sync();

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

    let bin_1 = NewBin::empty();
    let bin_2 = NewBin::from_static("Hello, slice!".as_bytes());
    let bin_3 = NewBin::from_given_vec(vec.clone());
    let bin_4 = NewSBin::from_given_vec(vec.clone()).un_sync();
    let bin_5 = NewBin::from_given_vec(generate_small_vec_that_fits_on_stack());
    let bin_6 = NewSBin::from_given_vec(generate_small_vec_that_fits_on_stack()).un_sync();

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

/// `into_vec` is allocation-free for empty binaries ... and for reference-counted binaries
/// (too large for stack) with only one reference.
fn into_vec_does_not_allocate() {
    // make sure this is too large for stack
    let len = STACK_BIN_LEN * 2;
    let bin_gen = BinGen::new(0, len);
    let vec = bin_gen.generate_to_vec();

    let bin_1 = NewBin::empty();
    let bin_2_allocates = NewBin::from_static("Hello, slice!".as_bytes());
    let bin_3 = NewBin::copy_from_slice(vec.as_slice());
    let bin_4 = NewSBin::copy_from_slice(vec.as_slice()).un_sync();
    let bin_5_allocates = NewBin::from_given_vec(generate_small_vec_that_fits_on_stack());
    let bin_6_allocates =
        NewSBin::from_given_vec(generate_small_vec_that_fits_on_stack()).un_sync();

    mem_scoped(&GLOBAL, &MaNoAllocNoReAlloc, || {
        // returns an empty vec
        assert_eq!(bin_1.into_vec(), Vec::new());
        // reference-counted binaries do not allocate unless there are multiple references pointing to them.
        bin_3.into_vec();
        // reference-counted binaries do not allocate unless there are multiple references pointing to them.
        bin_4.into_vec();
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

/// That's the maximum that fits on stack.
fn generate_small_vec_that_fits_on_stack() -> Vec<u8> {
    let bin_gen = BinGen::new(0, STACK_BIN_LEN);
    bin_gen.generate_to_vec()
}
