use abin::{AnyBin, NoVecCapShrink, UnSync, VecBin};
use utils::*;

pub mod utils;

#[test]
fn basic_vec() {
    for index in 0..1024 {
        test_vec_bin_as_simple_container(
            BinGen::new(index as u8, index as usize).generate_to_vec_shrink(0),
        );
    }
}

/// shows how to use `VecBin` as a simple container for `Vec<u8>` that never alters the
/// wrapped vector.
fn test_vec_bin_as_simple_container(vec: Vec<u8>) {
    let original_pointer = vec.as_ptr();
    let original_capacity = vec.capacity();

    let preserved_vec = vec.clone();

    // note: If we construct the vector using this configuration, it's just a simple container
    // for a vector - it never alters the wrapped vector (NoVecCapShrink &
    // allow_optimization = false are important).
    let bin = VecBin::from_with_cap_shrink::<NoVecCapShrink>(vec, false).un_sync();

    assert_eq!(bin.len(), preserved_vec.len());
    assert_eq!(bin.as_slice(), preserved_vec.as_slice());

    // we should be able to restore it
    let restored_vec = bin.into_vec();
    assert_eq!(restored_vec, preserved_vec);

    let restored_pointer = restored_vec.as_ptr();
    let restored_capacity = restored_vec.capacity();

    assert_eq!(original_pointer, restored_pointer);
    assert_eq!(restored_capacity, original_capacity);
}
