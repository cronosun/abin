use std::alloc::System;

use stats_alloc::{StatsAlloc, INSTRUMENTED_SYSTEM};

use abin::{AnyBin, AnyRc, ArcBin, Bin, RcBin, StaticBin, SyncBin, UnSyncRef};
use utils::*;

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

pub mod utils;

#[test]
fn test_re_integration() {
    static_re_integration();
    rc_re_integration::<RcBin, Bin>();
    rc_re_integration::<ArcBin, SyncBin>();
}

fn rc_re_integration<T: AnyRc<T = TBin>, TBin: AnyBin>() {
    let some_demo_slice = "This is some binary used for this test (the content does not \
    really matter - it just has to have some length)."
        .as_bytes();

    let bin_a = T::copy_from_slice(some_demo_slice);

    // this MUST NOT allocate
    mem_scoped(&GLOBAL, &MaNoAllocNoDealloc, || {
        rc_re_integration_stage_2::<T, TBin>(&bin_a, bin_a.as_slice(), some_demo_slice);
        rc_re_integration_stage_2::<T, TBin>(&bin_a, &bin_a.as_slice()[1..], &some_demo_slice[1..]);
        rc_re_integration_stage_2::<T, TBin>(
            &bin_a,
            &bin_a.as_slice()[5..20],
            &some_demo_slice[5..20],
        );
    });

    let something_unrelated = "unrelated binary".as_bytes();
    assert_eq!(None, bin_a.try_to_re_integrate(&something_unrelated));
}

fn rc_re_integration_stage_2<T: AnyRc<T = TBin>, TBin: AnyBin>(
    bin: &TBin,
    sub_item: &[u8],
    expected: &[u8],
) {
    let re_integrated_bin = bin.try_to_re_integrate(sub_item).unwrap();
    assert_eq!(expected, re_integrated_bin.as_slice());
}

fn static_re_integration() {
    let binary = "This is some static text. Hello, world!".as_bytes();
    let static_bin = StaticBin::from(binary);

    // must not allocate
    mem_scoped(&GLOBAL, &MaNoAllocNoDealloc, || {
        let sub_item_1 = binary;
        static_re_integration_stage_2(static_bin.un_sync_ref(), sub_item_1, binary);
        let sub_item_2 = &binary[1..];
        static_re_integration_stage_2(static_bin.un_sync_ref(), sub_item_2, &binary[1..]);
        let sub_item_3 = &binary[4..8];
        static_re_integration_stage_2(static_bin.un_sync_ref(), sub_item_3, &binary[4..8]);
    });

    // this cannot be re-integrated, since is a completely unrelated binary.
    let some_other_binary = "unrelated binary".as_bytes();
    assert_eq!(None, static_bin.try_to_re_integrate(some_other_binary));
}

fn static_re_integration_stage_2(bin: &Bin, sub_item: &[u8], expected: &[u8]) {
    let re_integrated_bin = bin.try_to_re_integrate(sub_item).unwrap();
    assert_eq!(expected, re_integrated_bin.as_slice());
}
