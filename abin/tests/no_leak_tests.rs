use std::alloc::System;
use std::cmp::max;

use stats_alloc::{StatsAlloc, INSTRUMENTED_SYSTEM};

use abin::{AnyBin, BinFactory, NewBin, NewSBin};
use utils::*;

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

pub mod utils;

#[test]
fn no_leak_tests() {
    basic_rc_memory::<NewBin>();
    basic_rc_memory::<NewSBin>();
}

fn basic_rc_memory<T: BinFactory>() {
    // some simple leak tests
    no_leak_1::<T>();
    no_leak_2::<T>();
    no_leak_3::<T>();
    no_leak_4::<T>();

    assert_no_leak::<T>();
}

/// Simple test that there's no leak.
fn no_leak_1<T: BinFactory>() {
    let vec_len = 1024 * 1024 * 32;
    mem_scoped(&GLOBAL, &MaNoLeak, || {
        let _vec1 = create_huge_allocation(vec_len);
    })
}

/// Simple test that there's no leak.
fn no_leak_2<T: BinFactory>() {
    let vec_len = 1024 * 1024 * 32;
    mem_scoped(&GLOBAL, &MaNoLeak, || {
        let vec1 = create_huge_allocation(vec_len);
        let _bin1 = T::from_given_vec(vec1);
    })
}

/// Simple test that there's no leak.
fn no_leak_3<T: BinFactory>() {
    let vec_len = 1024 * 1024 * 32;
    mem_scoped(&GLOBAL, &MaNoLeak, || {
        let vec1 = create_huge_allocation(vec_len);
        let bin1 = T::from_given_vec(vec1);
        let _bin11 = bin1.clone();
    })
}

/// Simple test that there's no leak.
fn no_leak_4<T: BinFactory>() {
    let vec_len = 1024 * 255;
    mem_scoped(&GLOBAL, &MaNoLeak, || {
        let vec1 = create_huge_allocation(vec_len);
        let bin1 = T::from_given_vec(vec1);
        {
            let _bin11 = bin1.clone();
        }
        let restored_vec = bin1.into_vec();
        let bin2 = T::from_given_vec(restored_vec);
        for idx in 0..20 {
            if idx % 2 == 0 {
                bin2.clone().into_vec();
            } else {
                let _ignored = bin2.clone();
            }
        }
    })
}

fn assert_no_leak<T: BinFactory>() {
    mem_scoped(&GLOBAL, &MaNoLeak, || {
        let vec_len = 1024 * 1024 * 32;

        let vec1 = create_huge_allocation(vec_len);
        let vec2 = create_huge_allocation(vec_len);
        let vec3 = create_huge_allocation(vec_len);

        let bin1 = T::from_given_vec(vec1);
        let bin2 = T::from_given_vec(vec2);
        let bin3 = T::from_given_vec(vec3);

        let bin11 = bin1.clone();
        let bin21 = bin2.clone();
        let _bin111 = bin11.clone();
        {
            let _bin32 = bin3.clone();
        }
        let _bin22 = bin21.clone();
        // should not allocate, since it's the only reference
        let _vec_bin3 = bin3.into_vec();

        let _vec_bin1 = bin1.into_vec();
    });
}

fn create_huge_allocation(number_of_bytes: usize) -> Vec<u8> {
    let mut huge_vec = Vec::with_capacity(number_of_bytes);
    let len = number_of_bytes;
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
