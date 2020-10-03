use std::alloc::System;
use std::cmp::max;

use stats_alloc::{Region, StatsAlloc, INSTRUMENTED_SYSTEM};

use abin::{AnyBin, AnyRc, ArcBin, Bin, RcBin, SBin};
use utils::*;

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

pub mod utils;

#[test]
fn basic_rc_memory_test() {
    basic_rc_memory::<ArcBin, SBin>();
    basic_rc_memory::<RcBin, Bin>();
}

fn basic_rc_memory<T: AnyRc<T = TBin>, TBin: AnyBin>() {
    // some simple leak tests
    no_leak_1::<T, TBin>();
    no_leak_2::<T, TBin>();
    no_leak_3::<T, TBin>();
    no_leak_4::<T, TBin>();

    into_vec_does_not_allocate_when_single_reference::<T, TBin>();
    assert_no_leak::<T, TBin>();
}

/// Simple test that there's no leak.
fn no_leak_1<T: AnyRc<T = TBin>, TBin: AnyBin>() {
    let vec_len = 1024 * 1024 * 32;
    mem_scoped(&GLOBAL, &MaNoLeak, || {
        let _vec1 = create_huge_allocation(vec_len, T::overhead_bytes());
    })
}

/// Simple test that there's no leak.
fn no_leak_2<T: AnyRc<T = TBin>, TBin: AnyBin>() {
    let vec_len = 1024 * 1024 * 32;
    mem_scoped(&GLOBAL, &MaNoLeak, || {
        let vec1 = create_huge_allocation(vec_len, T::overhead_bytes());
        let _bin1 = T::from_vec(vec1);
    })
}

/// Simple test that there's no leak.
fn no_leak_3<T: AnyRc<T = TBin>, TBin: AnyBin>() {
    let vec_len = 1024 * 1024 * 32;
    mem_scoped(&GLOBAL, &MaNoLeak, || {
        let vec1 = create_huge_allocation(vec_len, T::overhead_bytes());
        let bin1 = T::from_vec(vec1);
        let _bin11 = bin1.clone();
    })
}

/// Simple test that there's no leak.
fn no_leak_4<T: AnyRc<T = TBin>, TBin: AnyBin>() {
    let vec_len = 1024 * 255;
    mem_scoped(&GLOBAL, &MaNoLeak, || {
        let vec1 = create_huge_allocation(vec_len, T::overhead_bytes());
        let bin1 = T::from_vec(vec1);
        {
            let _bin11 = bin1.clone();
        }
        let restored_vec = bin1.into_vec();
        let bin2 = T::from_vec(restored_vec);
        for idx in 0..20 {
            if idx % 2 == 0 {
                bin2.clone().into_vec();
            } else {
                let _ignored = bin2.clone();
            }
        }
    })
}

fn into_vec_does_not_allocate_when_single_reference<T: AnyRc<T = TBin>, TBin: AnyBin>() {
    let vec_len = 1024 * 1024 * 32;
    let vec1 = create_huge_allocation(vec_len, T::overhead_bytes());
    let bin1 = T::from_vec(vec1);
    let _vec = mem_scoped(&GLOBAL, &MaNoAllocNoDealloc, || {
        // no allocation, since 'bin1' is single reference
        bin1.into_vec()
    });
}

fn assert_no_leak<T: AnyRc<T = TBin>, TBin: AnyBin>() {
    let mut reg = Region::new(&GLOBAL);
    let vec_len = 1024 * 1024 * 32;

    // expected: no change
    let change1 = reg.change_and_reset();

    let (change2, change3, change4) = {
        let vec1 = create_huge_allocation(vec_len, T::overhead_bytes());
        let vec2 = create_huge_allocation(vec_len, T::overhead_bytes());
        let vec3 = create_huge_allocation(vec_len, T::overhead_bytes());

        let bin1 = T::from_vec(vec1);
        let bin2 = T::from_vec(vec2);
        let bin3 = T::from_vec(vec3);

        // expected: about 3 * vec_len (the size of the 3 vectors)
        let change2 = reg.change_and_reset();

        let bin11 = bin1.clone();
        let bin21 = bin2.clone();
        let _bin111 = bin11.clone();
        {
            let _bin32 = bin3.clone();
        }
        let _bin22 = bin21.clone();
        // should not allocate, since it's the only reference
        let _vec_bin3 = bin3.into_vec();

        // expected: no change (since cloning does not allocate; into_vec does not allocate
        // if single reference).
        let change3 = reg.change_and_reset();

        let _vec_bin1 = bin1.into_vec();
        // expected: about 1 * vec_len (since bin1 still has references).
        let change4 = reg.change_and_reset();

        (change2, change3, change4)
    };
    // expected: about -(4 * vec_len)
    let change5 = reg.change_and_reset();

    assert!(change1.bytes_allocated == 0 && change1.bytes_deallocated == 0);
    assert!(change2.bytes_allocated == 100663296 && change2.bytes_deallocated == 0);
    assert!(change3.bytes_allocated == 0 && change3.bytes_deallocated == 0);
    assert!(change4.bytes_allocated == 33554432 && change4.bytes_deallocated == 0);
    assert!(change5.bytes_allocated == 0 && change5.bytes_deallocated == 100663296 + 33554432);
}

fn create_huge_allocation(
    number_of_bytes: usize,
    remaining_capacity_for_rc_overhead: usize,
) -> Vec<u8> {
    let mut huge_vec = Vec::with_capacity(number_of_bytes);
    let len = number_of_bytes - remaining_capacity_for_rc_overhead;
    unsafe {
        huge_vec.set_len(len);
    }

    let mut index = 0;
    let step_size = max(len / 1024, 10);
    // fill the buffer so the OS needs to reserve the pages.
    loop {
        if index >= len {
            break;
        }
        huge_vec[index] = index as u8;
        index = index + step_size;
    }
    huge_vec
}
