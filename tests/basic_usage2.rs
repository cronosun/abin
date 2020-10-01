use abin::{AnyBin, AnyRc, ArcBin, Bin, EmptyBin, RcBin, StaticBin, VecBin};

#[test]
pub fn usage() {
    let example_slice = "This is some binary used for the following examples.".as_bytes();

    // empty binary, stack-only.
    let bin1 = EmptyBin::new();
    // small binary; stack-only.
    let bin2 = RcBin::copy_from_slice(&example_slice[2..5]);
    // reference-counted binary (non-synchronized; like Rc);
    let bin3 = RcBin::copy_from_slice(example_slice);
    // reference-counted binary (synchronized; like Arc);
    let bin4 = ArcBin::from_vec(example_slice.to_vec());
    // binary backed by a Vec<u8>.
    let bin5 = VecBin::from_vec(example_slice.to_vec(), false);
    // no allocation for static data.
    let bin6 = StaticBin::from(example_slice);

    use_bin(bin1.into(), &[]);
    use_bin(bin2, &example_slice[2..5]);
    use_bin(bin3, example_slice);
    use_bin(bin4.into(), example_slice);
    use_bin(bin5.into(), example_slice);
    use_bin(bin6.into(), example_slice);
}

pub fn use_bin(bin: Bin, expected: &[u8]) {
    assert_eq!(expected.len(), bin.len());
    assert_eq!(expected, bin.as_slice());
    let cloned: Bin = bin.clone();
    assert_eq!(bin, cloned);
    let slice: Option<Bin> = bin.slice(1..3);
    if let Some(slice) = slice {
        assert_eq!(2, slice.len());
    }
    let vec_from_bin = bin.into_vec();
    assert_eq!(cloned.as_slice(), vec_from_bin.as_slice());
}
