use abin::{AnyBin, AnyRc, ArcBin, Bin, RcBin, StackBin, SyncBin, VecBin};

use stats_alloc::{StatsAlloc, INSTRUMENTED_SYSTEM};
use std::alloc::System;

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

pub mod utils;
use utils::*;

#[test]
pub fn stack_memory() {
    no_alloc();
    rc_uses_stack_no_alloc::<RcBin, Bin>();
    rc_uses_stack_no_alloc::<ArcBin, SyncBin>();
    vec_uses_stack_no_alloc();
}

/// Does not stack-allocate
fn no_alloc() {
    let slice = [15u8; StackBin::max_len()];
    mem_scoped(&GLOBAL, &MaNoAllocation, || {
        StackBin::try_from(&slice).expect("Max len must be stack-allocated");
    });
    mem_scoped(&GLOBAL, &MaNoAllocation, || {
        StackBin::try_from(&[]).expect("Empty must be stack-allocated");
    });
}

/// rc also uses stack and does not allocate for small binaries.
fn rc_uses_stack_no_alloc<T: AnyRc<T = TBin>, TBin: AnyBin>() {
    let slice = [15u8; StackBin::max_len()];
    mem_scoped(&GLOBAL, &MaNoAllocation, || {
        T::copy_from_slice(&slice);
    });
    mem_scoped(&GLOBAL, &MaNoAllocation, || {
        T::copy_from_slice(&[]);
    });
}

/// vec also uses stack and does not allocate for small binaries.
fn vec_uses_stack_no_alloc() {
    let slice = [15u8; StackBin::max_len()];
    let vec = slice.to_vec();
    mem_scoped(&GLOBAL, &MaOnlyDeAllocation, || {
        VecBin::from_vec(vec, true);
    });
    mem_scoped(&GLOBAL, &MaNoAllocation, || {
        VecBin::from_vec(Vec::new(), true);
    });
}
