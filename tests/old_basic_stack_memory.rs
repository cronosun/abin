use std::alloc::System;

use stats_alloc::{StatsAlloc, INSTRUMENTED_SYSTEM};

use abin::{Factory, NewBin, NewSBin};
use utils::*;

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

pub mod utils;

/// take a "conservative" length (depending on the platform; 32-bit; 64-bit this
/// is different; but 2 should be small enough for any platform).
const FITS_STACK: usize = 2;

#[test]
pub fn stack_memory() {
    copy_from_slice::<NewBin>();
    copy_from_slice::<NewSBin>();
    from_vec::<NewBin>();
    from_vec::<NewSBin>();
    from_iter::<NewBin>();
    from_iter::<NewSBin>();
}

/// rc also uses stack and does not allocate for small binaries.
fn copy_from_slice<T: Factory>() {
    let slice = [15u8; FITS_STACK];
    mem_scoped(&GLOBAL, &MaNoAllocNoDealloc, || {
        T::copy_from_slice(&slice);
    });
}

/// vec also uses stack and does not allocate for small binaries.
fn from_vec<T: Factory>() {
    let slice = [15u8; FITS_STACK];
    let vec = slice.to_vec();
    mem_scoped(&GLOBAL, &MaNoAllocNoReAlloc, || {
        T::from_given_vec(vec);
    });
}

/// vec also uses stack and does not allocate for small binaries.
fn from_iter<T: Factory>() {
    let slice = [15u8; FITS_STACK];
    let vec = Vec::from(&slice as &[u8]);
    mem_scoped(&GLOBAL, &MaNoAllocNoReAlloc, || {
        T::from_iter(vec);
    });
}
