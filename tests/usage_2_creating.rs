use std::ops::Deref;

use abin::{BinSegment, Factory, NewBin, NewStr, SegmentsSlice, StrFactory};

const MY_STATIC_STR: &'static str = "This is some static string; this is the content of the binary.";

#[test]
fn create_from_static() {
    let static_bin = NewBin::from_static(MY_STATIC_STR.as_bytes());
    assert_eq!(MY_STATIC_STR.as_bytes(), static_bin.deref());
    let static_str = NewStr::from_static(MY_STATIC_STR);
    assert_eq!(MY_STATIC_STR, static_str.as_str());
}

#[test]
fn create_empty() {
    let empty_bin = NewBin::empty();
    assert!(empty_bin.is_empty());
    assert_eq!(0, empty_bin.len());

    let empty_str = NewStr::empty();
    assert!(empty_str.is_empty());
    assert_eq!(0, empty_str.len());

    assert_eq!(&empty_bin, empty_str.as_bin());
}

#[test]
fn create_from_non_static() {
    let string = MY_STATIC_STR.to_owned();
    let non_static_src = string.as_str();

    let non_static_bin = NewBin::copy_from_slice(non_static_src.as_bytes());
    assert_eq!(non_static_src.as_bytes(), non_static_bin.deref());
    let non_static_str = NewStr::copy_from_str(non_static_src);
    assert_eq!(non_static_src, non_static_str.as_str());
}

#[test]
fn create_from_iterator() {
    let string = MY_STATIC_STR.to_owned();
    let non_static_src = string.as_str();

    let iter_bin_1 = NewBin::from_iter(non_static_src.as_bytes().iter().cloned());
    assert_eq!(non_static_src.as_bytes(), iter_bin_1.deref());
    let iter_bin_2 = NewBin::from_iter((0..50).into_iter().map(|i| i as u8));
    assert_eq!(50, iter_bin_2.len());

    let iter_str_1 = NewStr::from_utf8_iter(non_static_src.as_bytes().iter().cloned()).expect("invalid utf-8");
    assert_eq!(non_static_src, iter_str_1.deref());
    let iter_str_2 = NewStr::from_utf8_iter((97..103).into_iter().map(|i| i as u8)).expect("invalid utf-8");
    assert_eq!("abcdef", iter_str_2.deref());
}

/// Demonstrates how to construct a binary from multiple segments (efficiently). Should
/// need just one single allocation.
#[test]
fn bin_create_from_segments() {
    // "Hello, "
    let src_hello = &[72u8, 101u8, 108u8, 108u8, 111u8, 44u8, 32u8];
    // "world"
    let src_world = &[119u8, 111u8, 114u8, 108u8, 100u8];
    // "!"
    let src_exclamation = &[33u8];

    let segment1 = BinSegment::Static(src_hello);
    let segment2 = BinSegment::Slice(src_world);
    // note: this 'BinSegment::Empty' is not required (has no effect), just for demonstration.
    let segment3 = BinSegment::Empty;
    let segment4 = BinSegment::Bin(NewBin::from_static(src_exclamation));

    let resulting_bin = NewBin::from_segments(SegmentsSlice::new(&mut [segment1, segment2, segment3, segment4]));
    assert_eq!(src_hello.len() + src_world.len() + src_exclamation.len(), resulting_bin.len());
    assert_eq!("Hello, world!".as_bytes(), resulting_bin.deref());
}

#[test]
fn str_create_from_segments() {
    // TODO
    unimplemented!()
}