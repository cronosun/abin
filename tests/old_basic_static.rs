use abin::{AnyBin, BinFactory, NewBin};

const EMPTY: &[u8] = &[];
const ONE: &[u8] = &[15];
const MANY: &[u8] = &[12, 44, 15, 8, 255, 0, 254];
const LARGE: &str = "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy \
eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero \
eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata \
sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, \
sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam \
voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, \
no sea takimata sanctus est Lorem ipsum dolor sit amet.";

#[test]
fn basic_static() {
    test_static(EMPTY);
    test_static(ONE);
    test_static(MANY);
    test_static(LARGE.as_bytes());
}

fn test_static(slice: &'static [u8]) {
    let slice_ptr = slice.as_ptr();
    let bin = NewBin::from_static(slice);
    assert_eq!(slice.len(), bin.len());
    assert_eq!(slice, bin.as_slice());

    // non-empty slices must point to the same memory location
    if !slice.is_empty() {
        let bin_slice_ptr = bin.as_slice().as_ptr();
        assert_eq!(slice_ptr, bin_slice_ptr);
    }
}
