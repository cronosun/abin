use std::iter::FromIterator;
use std::ops::Deref;

use abin::{AnyBin, AnyStr, Bin, BinFactory, NewBin, NewStr, Str, StrFactory};

#[test]
fn usage_basics() {
    // static binary / static string
    let static_bin: Bin = NewBin::from_static("I'm a static binary, hello!".as_bytes());
    let static_str: Str = NewStr::from_static("I'm a static binary, hello!");
    assert_eq!(&static_bin, static_str.as_bin());
    assert_eq!(static_str.as_str(), "I'm a static binary, hello!");
    // non-static (but small enough to be stored on the stack)
    let hello_bin: Bin = NewBin::from_iter([72u8, 101u8, 108u8, 108u8, 111u8].iter().copied());
    let hello_str: Str = NewStr::copy_from_str("Hello");
    assert_eq!(&hello_bin, hello_str.as_bin());
    assert_eq!(hello_str.as_ref() as &str, "Hello");

    // operations for binaries / strings

    // length (number of bytes / number of utf-8 bytes)
    assert_eq!(5, hello_bin.len());
    assert_eq!(5, hello_str.len());
    // is_empty
    assert_eq!(false, hello_bin.is_empty());
    assert_eq!(false, hello_str.is_empty());
    // as_slice / as_str / deref / as_bin
    assert_eq!(&[72u8, 101u8, 108u8, 108u8, 111u8], hello_bin.as_slice());
    assert_eq!("Hello", hello_str.as_str());
    assert_eq!("Hello", hello_str.deref());
    assert_eq!(&hello_bin, hello_str.as_bin());
    // slice
    assert_eq!(
        NewBin::from_static(&[72u8, 101u8]),
        hello_bin.slice(0..2).unwrap()
    );
    assert_eq!(NewStr::from_static("He"), hello_str.slice(0..2).unwrap());
    // clone
    assert_eq!(hello_bin.clone(), hello_bin);
    assert_eq!(hello_str.clone(), hello_str);
    // compare
    assert!(NewBin::from_static(&[255u8]) > hello_bin);
    assert!(NewStr::from_static("Z") > hello_str);
    // convert string into binary and binary into string
    let hello_bin_from_str: Bin = hello_str.clone().into_bin();
    assert_eq!(hello_bin_from_str, hello_bin);
    let hello_str_from_bin: Str = AnyStr::from_utf8(hello_bin.clone()).expect("invalid utf8!");
    assert_eq!(hello_str_from_bin, hello_str);
    // convert into Vec<u8> / String
    assert_eq!(
        Vec::from_iter([72u8, 101u8, 108u8, 108u8, 111u8].iter().copied()),
        hello_bin.into_vec()
    );
    assert_eq!("Hello".to_owned(), hello_str.into_string());
}
