use std::alloc::System;

use stats_alloc::{StatsAlloc, INSTRUMENTED_SYSTEM};

use abin::{BinFactory, NewBin, NewSBin};
use utils::*;

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

pub mod utils;

// that's the maximum that fits onto the stack.
const FITS_STACK: usize = 3 * core::mem::size_of::<usize>() - 1;

#[test]
fn stack_memory() {
    mem_scoped(&GLOBAL, &MaNoLeak, || {
        copy_from_slice::<NewBin>();
        copy_from_slice::<NewSBin>();
        from_vec::<NewBin>();
        from_vec::<NewSBin>();
        from_iter::<NewBin>();
        from_iter::<NewSBin>();
    });
}

/// rc also uses stack and does not allocate for small binaries.
fn copy_from_slice<T: BinFactory>() {
    let slice = [15u8; FITS_STACK];
    mem_scoped(&GLOBAL, &MaNoAllocNoDealloc, || {
        T::copy_from_slice(&slice);
    });
}

/// vec also uses stack and does not allocate for small binaries.
fn from_vec<T: BinFactory>() {
    let slice = [15u8; FITS_STACK];
    let vec = slice.to_vec();
    mem_scoped(&GLOBAL, &MaNoAllocNoReAlloc, || {
        T::from_given_vec(vec);
    });
}

/// iter also uses stack and does not allocate for small binaries.
fn from_iter<T: BinFactory>() {
    let slice = [15u8; FITS_STACK];
    let vec = Vec::from(&slice as &[u8]);
    mem_scoped(&GLOBAL, &MaNoAllocNoReAlloc, || {
        T::from_iter(vec);
    });
}
