use abin::{AnyBin, VecBin};
use utils::*;
pub mod utils;

#[test]
fn basic_vec() {
    for index in 0..1024 {
        test_vec(BinGen::new(index as u8, index as usize).generate_to_vec());
    }
}

fn test_vec(vec: Vec<u8>) {
    let original_pointer = vec.as_ptr();
    let original_capacity = vec.capacity();

    let preserved_vec = vec.clone();
    let bin = VecBin::from(vec).un_sync();

    assert_eq!(bin.len(), preserved_vec.len());
    assert_eq!(bin.as_slice(), preserved_vec.as_slice());

    // we should be able to restore it
    let restored_vec = bin.into_vec();
    assert_eq!(restored_vec, preserved_vec);

    // the restored vector should be identical to the original vector -> but this is only true
    // for larger vectors. Smaller vectors are converted to stack-only binaries.
    if preserved_vec.len() > 50 {
        let restored_pointer = restored_vec.as_ptr();
        let restored_capacity = restored_vec.capacity();

        assert_eq!(original_pointer, restored_pointer);
        assert_eq!(restored_capacity, original_capacity);
    }
}