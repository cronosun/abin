use std::alloc::System;
use std::cmp::max;
use std::ops::Deref;

use stats_alloc::{INSTRUMENTED_SYSTEM, Region, StatsAlloc};

use abin::{AnyBin, AnyRc, ArcBin, Bin, NoVecCapShrink, RcBin, SyncBin};
use bin_gen::BinGen;

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

/// small vectors are optimized (stack only)... for those the tests would fail.
const SAFE_SIZE: usize = 50;

mod bin_gen;

#[test]
fn basic_rc_non_sync() {
    basic_rc::<RcBin, Bin>();
}

#[test]
fn basic_rc_sync() {
    basic_rc::<ArcBin, SyncBin>();
}

fn basic_rc<T: AnyRc<T=TBin>, TBin: AnyBin>() {
    for index in SAFE_SIZE..1024 {
        let bin_gen = BinGen::new(index as u8, index as usize);
        extracting_returns_same_memory_location::<T, TBin>(&bin_gen);
        into_vec_with_more_than_one_ref_count::<T, TBin>(&bin_gen);
    }
    assert_no_leak::<T, TBin>();
}

/// When a rc-bin is converted into a vec and there is more than one ref count, a copy must
/// be returned.
fn into_vec_with_more_than_one_ref_count<T: AnyRc<T=TBin>, TBin: AnyBin>(bin_gen: &BinGen) {
    let mut vec = bin_gen.generate_to_vec();
    // make sure we have enough capacity (since if not, the system is allowed /
    // forced to change memory location).
    vec.reserve(T::overhead_bytes());
    let original_clone = vec.clone();
    assert!(vec.len() >= SAFE_SIZE, "This test only works for large vectors (since small vectors \
    might get optimized; see StackBin).");
    let original_address = vec.as_ptr() as usize;
    let original_capacity = vec.capacity();

    let bin_1 = T::from_with_cap_shrink::<NoVecCapShrink>(vec);
    let bin_2 = bin_1.clone();
    let bin_3 = bin_2.clone();
    let bin_4 = bin_1.clone();
    let bin_5 = bin_4.clone();

    // here we must have different addresses, since bin has a ref count > 0.
    assert_ne!(original_address, bin_1.into_vec().as_slice().as_ptr() as usize);
    assert_ne!(original_address, bin_2.into_vec().as_slice().as_ptr() as usize);
    assert_ne!(original_address, bin_3.into_vec().as_slice().as_ptr() as usize);
    assert_ne!(original_address, bin_4.into_vec().as_slice().as_ptr() as usize);

    // now the last one... this must return the same address.
    let restored_vec = bin_5.into_vec();
    assert_eq!(original_address, restored_vec.as_slice().as_ptr() as usize);
    // also capacity and content must be equal
    assert_eq!(restored_vec, original_clone);
    assert_eq!(restored_vec.capacity(), original_capacity);
}

/// When a rc-bin is converted into a vec and there's only one single reference, this
/// must return the same vector (same address).
fn extracting_returns_same_memory_location<T: AnyRc<T=TBin>, TBin: AnyBin>(bin_gen: &BinGen) {
    let (original_capacity, original_address, original_clone, bin) = {
        let mut vec = bin_gen.generate_to_vec();
        // make sure we have enough capacity (since if not, the system is allowed /
        // forced to change memory location).
        vec.reserve(T::overhead_bytes());

        let original_clone = vec.clone();
        assert!(vec.len() >= SAFE_SIZE, "This test only works for large vectors (since small vectors \
    might get optimized; see StackBin).");
        let original_address = vec.as_ptr() as usize;
        let original_capacity = vec.capacity();
        let bin = T::from_with_cap_shrink::<NoVecCapShrink>(vec);
        (original_capacity, original_address, original_clone, bin)
    };

    // first make sure we have the same content
    assert_eq!(bin.as_slice(), original_clone.deref());
    assert_eq!(bin.len(), original_clone.len());

    // now make sure that the addresses are the same (e.g. no memory copy happened).
    assert_eq!(bin.as_slice().as_ptr() as usize, original_address);

    // and since there's only one reference, we must be able to get back the original vector.
    let restored_vec = bin.into_vec();
    assert_eq!(restored_vec.as_slice().as_ptr() as usize, original_address);
    assert_eq!(restored_vec.capacity(), original_capacity, "capacity mismatch");
}

fn assert_no_leak<T: AnyRc<T=TBin>, TBin: AnyBin>() {
    let mut reg = Region::new(&GLOBAL);
    let vec_len = 1024 * 1024 * 32;

    // expected: no change
    let change1 = reg.change_and_reset();

    let (change2, change3, change4) = {
        let vec1 = create_huge_allocation(vec_len, T::overhead_bytes());
        let vec2 = create_huge_allocation(vec_len, T::overhead_bytes());
        let vec3 = create_huge_allocation(vec_len,T::overhead_bytes());

        let bin1 = T::from(vec1);
        let bin2 = T::from(vec2);
        let bin3 = T::from(vec3);

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

    println!("1: {:?}", change1);
    println!("2: {:?}", change2);
    println!("3: {:?}", change3);
    println!("4: {:?}", change4);
    println!("5: {:?}", change5);

    assert!(change1.bytes_allocated == 0 && change1.bytes_deallocated == 0);
    assert!(change2.bytes_allocated == 100663296 && change2.bytes_deallocated == 0);
    //assert!(change3.bytes_allocated == 0 && change3.bytes_deallocated == 0); // TODO: There's a bug
    assert!(change4.bytes_allocated == 33554425 && change4.bytes_deallocated == 0);
    //assert!(change5.bytes_allocated == 0 && change5.bytes_deallocated == 100663296 + 33554425); // TODO: There's a bug
}

fn create_huge_allocation(number_of_bytes: usize, remaining_capacity_for_rc_overhead : usize) -> Vec<u8> {
    let mut huge_vec = Vec::with_capacity(number_of_bytes);
    let len = number_of_bytes - remaining_capacity_for_rc_overhead;
    unsafe { huge_vec.set_len(len); }

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