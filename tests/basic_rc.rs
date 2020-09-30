use std::ops::Deref;

use abin::{AnyBin, AnyRc, ArcBin, Bin, NoVecCapShrink, RcBin, SyncBin};
use utils::*;

pub mod utils;

/// small vectors are optimized (stack only)... for those the tests would fail.
const SAFE_SIZE: usize = 50;

#[test]
fn basic_rc_non_sync() {
    basic_rc::<RcBin, Bin>();
}

#[test]
fn basic_rc_sync() {
    basic_rc::<ArcBin, SyncBin>();
}

fn basic_rc<T: AnyRc<T = TBin>, TBin: AnyBin>() {
    for index in SAFE_SIZE..1024 {
        let bin_gen = BinGen::new(index as u8, index as usize);
        extracting_returns_same_memory_location::<T, TBin>(&bin_gen);
        into_vec_with_more_than_one_ref_count::<T, TBin>(&bin_gen);
    }
}

/// When a rc-bin is converted into a vec and there is more than one ref count, a copy must
/// be returned.
fn into_vec_with_more_than_one_ref_count<T: AnyRc<T = TBin>, TBin: AnyBin>(bin_gen: &BinGen) {
    // make sure we have enough capacity (since if not, the system is allowed /
    // forced to change memory location).
    let vec = bin_gen.generate_to_vec_shrink(T::overhead_bytes());

    // clone: we need to compare content later.
    let original_clone = vec.clone();

    assert!(
        vec.len() >= SAFE_SIZE,
        "This test only works for large vectors (since small vectors \
    might get optimized; see StackBin)."
    );
    let original_address = vec.as_ptr() as usize;
    let original_capacity = vec.capacity();

    let bin_1 = T::from_with_cap_shrink::<NoVecCapShrink>(vec);
    let bin_2 = bin_1.clone();
    let bin_3 = bin_2.clone();
    let bin_4 = bin_1.clone();
    let bin_5 = bin_4.clone();

    // here we must have different addresses, since bin has a ref count > 0.
    assert_ne!(
        original_address,
        bin_1.into_vec().as_slice().as_ptr() as usize
    );
    assert_ne!(
        original_address,
        bin_2.into_vec().as_slice().as_ptr() as usize
    );
    assert_ne!(
        original_address,
        bin_3.into_vec().as_slice().as_ptr() as usize
    );
    assert_ne!(
        original_address,
        bin_4.into_vec().as_slice().as_ptr() as usize
    );

    // now the last one... this must return the same address.
    let restored_vec = bin_5.into_vec();
    assert_eq!(original_address, restored_vec.as_slice().as_ptr() as usize);
    // also capacity and content must be equal
    assert_eq!(restored_vec, original_clone);
    assert_eq!(restored_vec.capacity(), original_capacity);
}

/// When a rc-bin is converted into a vec and there's only one single reference, this
/// must return the same vector (same address).
fn extracting_returns_same_memory_location<T: AnyRc<T = TBin>, TBin: AnyBin>(bin_gen: &BinGen) {
    let (original_capacity, original_address, original_clone, bin) = {
        // make sure we have enough capacity (since if not, the system is allowed /
        // forced to change memory location).
        let vec = bin_gen.generate_to_vec_shrink(T::overhead_bytes());

        // crate a clone (so we can check content later).
        let original_clone = vec.clone();

        assert!(
            vec.len() >= SAFE_SIZE,
            "This test only works for large vectors (since small vectors \
    might get optimized; see StackBin)."
        );
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
    assert_eq!(
        restored_vec.capacity(),
        original_capacity,
        "capacity mismatch"
    );
}
