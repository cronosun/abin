use abin::{AnyBin, AnyStr, AnyStrUtf8Error, Bin, BinFactory, NewBin, NewStr, Str, StrFactory};

#[test]
fn slice() {
    let str1 = NewStr::from_static("Some text.");
    assert_eq!(
        "Some text.".get(5..9).unwrap(),
        str1.slice(5..9).unwrap().as_str()
    );

    let str2 = NewStr::from_static("Warning: Don't open the door!");
    assert_eq!("Don't open the door!", str2.slice(9..).unwrap().as_str());

    // out of bounds returns `None`
    let str3 = NewStr::from_static("HELLO");
    assert!(str3.slice(0..6).is_none());

    // invalid range returns none.
    let str4 = NewStr::from_static("HELLO");
    assert!(str4.slice(2..1).is_none())
}

/// A binary can always be converted to a string (as long as it's valid UTF-8).
#[test]
fn convert_from_utf8() {
    // valid utf-8
    let valid_utf8 = NewBin::from_static("Hello, world!".as_bytes());
    let bin_ptr = valid_utf8.as_slice().as_ptr();
    let string = Str::from_utf8(valid_utf8).unwrap();
    assert_eq!("Hello, world!", string.as_str());
    // no memory-copy / zero-allocation (still the same pointer).
    assert_eq!(bin_ptr, string.as_ptr());

    // invalid UTF-8
    let invalid_utf8 = NewBin::from_static(&[0xa0, 0xa1]);
    match Str::from_utf8(invalid_utf8) {
        Ok(_) => {
            panic!("Expected an error");
        }
        Err(err) => {
            // we get the original binary back.
            let (_, bin) = err.deconstruct();
            assert_eq!(&[0xa0, 0xa1], bin.as_slice());
        }
    }
}

/// A binary/string can always be converted back to a Vec/String.
///
/// The implementation guarantees zero-allocation / zero-copy.
#[test]
fn convert_to_vec_string() {
    let input_data = "This is some input data. It has some length to make sure it's \
    not stored on the stack (if stored on the stack, the test would fail - in that case a new vec \
    has to be allocated and the pointers would differ)";

    // with string:
    let str = NewStr::copy_from_str(input_data);
    // save the pointer to the data.
    let pointer = str.as_str().as_ptr();
    // now convert to String
    let string = str.into_string();
    assert_eq!(input_data, string.as_str());
    // still the same pointer.
    assert_eq!(pointer, string.as_ptr());

    // with binary:
    let bin = NewBin::copy_from_slice(input_data.as_bytes());
    // save the pointer to the data.
    let pointer = bin.as_slice().as_ptr();
    // now convert to Vec<u8>
    let vec = bin.into_vec();
    assert_eq!(input_data.as_bytes(), vec.as_slice());
    // still the same pointer.
    assert_eq!(pointer, vec.as_ptr());
}

#[test]
fn ord() {
    assert!(NewStr::from_static("aaa") < NewStr::from_static("az"));
    assert!(NewBin::from_static(&[15u8, 77u8, 20u8]) < NewBin::from_static(&[15u8, 78u8]));
}
