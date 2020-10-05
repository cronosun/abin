use std::ops::Deref;

use serde::de::Unexpected::NewtypeStruct;

use abin::{
    AnyBin, Bin, BinFactory, BinSegment, NewBin, NewStr, SegmentsSlice, Str, StrFactory, StrSegment,
};

const MY_STATIC_STR: &'static str =
    "This is some static string; this is the content of the binary.";

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

    let iter_str_1 =
        NewStr::from_utf8_iter(non_static_src.as_bytes().iter().cloned()).expect("invalid utf-8");
    assert_eq!(non_static_src, iter_str_1.deref());
    let iter_str_2 =
        NewStr::from_utf8_iter((97..103).into_iter().map(|i| i as u8)).expect("invalid utf-8");
    assert_eq!("abcdef", iter_str_2.deref());
}

/// Demonstrates how to construct a binary from multiple segments (efficiently). Should
/// need just one single allocation.
///
/// Note: Alternative is using a builder.
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

    let resulting_bin = NewBin::from_segments(SegmentsSlice::new(&mut [
        segment1, segment2, segment3, segment4,
    ]));
    assert_eq!(
        src_hello.len() + src_world.len() + src_exclamation.len(),
        resulting_bin.len()
    );
    assert_eq!("Hello, world!".as_bytes(), resulting_bin.deref());
}

/// Demonstrates how to construct a string from multiple segments (efficiently). Should
/// need just one single allocation.
#[test]
fn str_create_from_segments() {
    let src_hello = "Hello, ";
    let src_world = "world";
    let src_exclamation = "!";

    let segment1 = StrSegment::Static(src_hello);
    let segment2 = StrSegment::Slice(src_world);
    // note: this 'StrSegment::Empty' is not required (has no effect), just for demonstration.
    let segment3 = StrSegment::Empty;
    let segment4 = StrSegment::Str(NewStr::from_static(src_exclamation));

    let resulting_str = NewStr::from_segments(SegmentsSlice::new(&mut [
        segment1, segment2, segment3, segment4,
    ]));
    assert_eq!(
        src_hello.len() + src_world.len() + src_exclamation.len(),
        resulting_str.len()
    );
    assert_eq!("Hello, world!", resulting_str.deref());
}

#[test]
fn from_segment() {
    let bin_segment_1: BinSegment<'static, Bin> = (&[77u8, 90u8] as &'static [u8]).into();
    assert_eq!(
        &[77u8, 90u8] as &'static [u8],
        NewBin::from_segment(bin_segment_1).as_ref()
    );

    let bin_segment_2 = BinSegment::Static(&[77u8, 90u8] as &'static [u8]);
    assert_eq!(
        &[77u8, 90u8] as &'static [u8],
        NewBin::from_segment(bin_segment_2).as_ref()
    );

    let bin_segment_3 = BinSegment::from(&[77u8, 90u8] as &[u8]);
    assert_eq!(
        &[77u8, 90u8] as &[u8],
        NewBin::from_segment(bin_segment_3).as_ref()
    );
    let bin_segment_4 = BinSegment::Bytes128(158.into());
    assert_eq!(
        &[158u8] as &[u8],
        NewBin::from_segment(bin_segment_4).as_ref()
    );

    let str_segment_1: StrSegment<'static, Bin> = "MZ".into();
    assert_eq!("MZ", NewStr::from_segment(str_segment_1).as_str());

    let str_segment_2 = StrSegment::Static("MZ");
    assert_eq!("MZ", NewStr::from_segment(str_segment_2).as_str());

    let str_segment_3 = StrSegment::from("MZ");
    assert_eq!("MZ", NewStr::from_segment(str_segment_3).as_str());

    let str_segment_4 = StrSegment::Char('💣');
    assert_eq!("💣", NewStr::from_segment(str_segment_4).as_str());
}

/// If you need to concatenate strings, use something like this. This guarantees one single
/// allocation.
#[test]
fn create_dynamic_greeting_with_just_one_allocation() {
    assert_eq!(
        "Hello, Claus!",
        efficient_greeting_string(GreetingCfg::GreetClaus).as_str()
    );
    assert_eq!(
        "Hello, Julia!",
        efficient_greeting_string(GreetingCfg::GreetJulia).as_str()
    );
    assert_eq!(
        "Hello, world!",
        efficient_greeting_string(GreetingCfg::GreetSomebodyNamed(NewStr::from_static(
            "world"
        )))
        .as_str()
    );
    assert_eq!(
        "Hello, !",
        efficient_greeting_string(GreetingCfg::Empty).as_str()
    );
    assert_eq!(
        "Hello, world and welt!",
        efficient_greeting_string(GreetingCfg::GreetTwo(
            NewStr::from_static("world"),
            NewStr::from_static("welt"),
        ))
        .as_str()
    );
    // even zero-allocation:
    assert_eq!(
        "Hello, zero-allocation!",
        efficient_greeting_string(GreetingCfg::CustomGreeting(NewStr::from_static(
            "Hello, zero-allocation!"
        )))
        .as_str()
    );
}

fn efficient_greeting_string(cfg: GreetingCfg) -> Str {
    let prefix = StrSegment::Static("Hello, ");
    let exclamation = StrSegment::Static("!");
    match cfg {
        GreetingCfg::GreetClaus => {
            let name = StrSegment::Static("Claus");
            NewStr::from_segments(SegmentsSlice::new(&mut [prefix, name, exclamation]))
        }
        GreetingCfg::GreetJulia => {
            let name = StrSegment::Static("Julia");
            NewStr::from_segments(SegmentsSlice::new(&mut [prefix, name, exclamation]))
        }
        GreetingCfg::GreetSomebodyNamed(name) => {
            let name = StrSegment::Str(name);
            NewStr::from_segments(SegmentsSlice::new(&mut [prefix, name, exclamation]))
        }
        GreetingCfg::Empty => NewStr::from_segments(SegmentsSlice::new(&mut [
            prefix,
            StrSegment::Empty,
            exclamation,
        ])),
        GreetingCfg::GreetTwo(name1, name2) => {
            let name1 = StrSegment::Str(name1);
            let name2 = StrSegment::Str(name2);
            let and = StrSegment::Static(" and ");
            NewStr::from_segments(SegmentsSlice::new(&mut [
                prefix,
                name1,
                and,
                name2,
                exclamation,
            ]))
        }
        GreetingCfg::CustomGreeting(str) => {
            // alternative: just return the given `str`.
            let str = StrSegment::Str(str);
            // this won't allocate.
            NewStr::from_segments(SegmentsSlice::new(&mut [str]))
        }
    }
}

enum GreetingCfg {
    GreetClaus,
    GreetJulia,
    GreetSomebodyNamed(Str),
    Empty,
    GreetTwo(Str, Str),
    CustomGreeting(Str),
}

/// How to construct a `Bin` from `Vec<u8>` or a `Str` from a `String`.
///
/// Note: Whenever possible try to avoid this, since this might need an allocation /
/// memory-copy (depending on the capacity of `Vec<u8>` / `String`). Only use this if you're
/// given a `Vec<u8>` / `String` from some library you don't control.
#[test]
fn from_vec_or_string() {
    let src_string = "This is the content of the string/binary; Hello, world!";

    let vec = src_string.to_owned().into_bytes();
    let bin = NewBin::from_given_vec(vec);
    assert_eq!(src_string.as_bytes(), bin.as_slice());

    let string = src_string.to_owned();
    let any_str = NewStr::from_given_string(string);
    assert_eq!(src_string, any_str.as_str());

    assert_eq!(&bin, any_str.as_ref());
}
