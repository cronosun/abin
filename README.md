# abin

[![Crates.io](https://img.shields.io/crates/v/abin.svg)](https://crates.io/crates/abin)
[![Docs.rs](https://docs.rs/abin/badge.svg)](https://docs.rs/abin)
[![CI](https://github.com/cronosun/abin/workflows/Continuous%20Integration/badge.svg)](https://github.com/cronosun/abin/actions)
[![Coverage Status](https://coveralls.io/repos/github/cronosun/abin/badge.svg?branch=master)](https://coveralls.io/github/cronosun/abin?branch=master)
[![Rust GitHub Template](https://img.shields.io/badge/Rust%20GitHub-Template-blue)](https://rust-github.github.io/)

A library for working with binaries and strings. The library tries to avoid heap-allocations / memory-copy whenever possible by automatically choosing a reasonable strategy (stack for small binaries; static-lifetime-binary or reference-counting). It's easy to use (no lifetimes; the binary type is sized), `Send + Sync` is optional (thus no synchronization overhead), provides optional serde support and has a similar API for strings and binaries. Custom binary/string types can be implemented for fine-tuning.

Libraries that provide similar functionality:

 * [https://github.com/tokio-rs/bytes](https://github.com/tokio-rs/bytes)
 * [https://github.com/rust-analyzer/smol_str](https://github.com/rust-analyzer/smol_str)

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

# Details

## Usage

```toml
[dependencies]
abin = "*"
```

```rust
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
```

## Notable structs, traits and types & naming

Interfaces: 
  * `Bin`: Binary (it's a struct).
  * `SBin`: Synchronized binary (it's a struct).
  * `Str`: String (`type Str = AnyStr<Bin>`)
  * `SStr` Synchronized string (`type SStr = AnyStr<SBin>`).
 
Factories provided by the default implementation: 
  * `NewBin`: Creates `Bin`.
  * `NewSBin`: Creates `SBin`.
  * `NewStr`: Creates `Str`.
  * `NewSStr`: Creates `SStr`. 

See also:
  * `AnyBin`: Trait implemented by `Bin` and `SBin`.
  * `AnyStr`: See `Str` and `SStr`; string backed by either `Bin` or `SBin`.
  * `BinFactory`: Factory trait implemented by `NewBin` and `NewSBin`.
  * `StrFactory`: Factory trait implemented by `NewStr` and `NewSStr`.

## Learn

See the example tests:

* [tests/usage_1_basics.rs](tests/usage_1_basics.rs): Basic usage.
* [tests/usage_2_creating.rs](tests/usage_2_creating.rs): How to create binaries and strings.
* [tests/usage_3_builder.rs](tests/usage_3_builder.rs): How to use the builder to create binaries / strings.
* [tests/usage_4_operations.rs](tests/usage_4_operations.rs): Operations provided by binaries and strings, such as slicing, converting binaries to strings and converting binaries/strings to `Vec<u8>` and `String`.
* [tests/usage_5_boo.rs](tests/usage_5_boo.rs): "Borrowed-Or-Owned" (boo), alternative to `Cow` that works with types that don't implement `ToOwned`.
* [tests/usage_6_serde_boo.rs](tests/usage_6_serde_boo.rs): Use `Boo` with serde.
* [tests/usage_7_serde_ri.rs](tests/usage_7_serde_ri.rs): Use serde with re-integration (also see *Questions and Answers*).
* [tests/usage_8_send_sync.rs](tests/usage_8_send_sync.rs): Synchronized (`Send + Sync`) and non-synchronized binaries / strings.
* [tests/usage_9_re_integration.rs](tests/usage_9_re_integration.rs): Re-integration (also see *Questions and Answers*)

## Maturity

It's quite young (development started in October 2020). The main functionality has been implemented. Things I might do:

 * API refinement.
 * Tests using `loom` / rayon / more tests.

## Questions and Answers

**There's already other crates with similar functionality, why another one? / Features**

This crate provides some features that cannot be found in other crates (or not all of them):

 * Provides support for binaries **and** strings; the API for strings mirrors the binary-API closely.
 * Binaries/strings are not synchronized when not needed (synchronization is optional).
 * Custom implementations are possible.
 * Small binaries/strings are stored on the stack.
 * Support for serde zero-allocation deserialization to owned types (in some situations).
 * Efficient cloning (usually zero-allocation / zero-copy).
 * Efficient slicing to owned types (slice from `Bin`/`Str` to `Bin`/`Str`) (usually zero-allocation / zero-copy).
 * Guaranteed zero-allocation/zero-copy borrowed slicing (slice from `Bin`/`Str` to `&[u8]`/`&str`).
 * Provide everything to be used as keys in maps / serde support.

**Why `NewBin`, `NewStr`? what's this?**

Why `let string = NewStr::from_static("Hello")` instead of just `let string = Str::from_static("Hello")` (or implement `From<&str> for Str`)? This is due to the decision to decouple the interface from the implementation. The `Str` is the interface, whereas `NewStr` is the factory of the built-in implementation. This library is designed to be extensible; you can provide your own implementation, tweaked for your use case.

**How does the default-implementation `NewBin` / `NewStr` work?**

  * Small binaries are stored on the stack. Up to `3 * sizeof(word) - 1` bytes; that's 23 bytes on a 64-bit platform. For reference, the string `Hello, world!` only takes 13 bytes and could easily be stored on the stack.
  * Static binaries are just pointers to the actual data (so stack-only).
  * Larger binaries are usually (*1) reference-counted. (*1: There's a tweak to change this behaviour, see `GivenVecConfig`). The reference-counter is stored inside the vector-data. This has those advantages:
    * It's possible to create a `Bin` from `Vec<u8>` without allocation (if `Vec<u8>` has some capacity left for the reference-counter) - something which is not possible by using `Rc<[u8]>`.
    * ...at the same time (unlike `Rc<Vec<u8>>`) no second indirection is introduced.

The only difference between `NewBin` and `NewSBin` is the reference-counted binaries: `SBin` created by `NewSBin` have a synchronized reference counter (`AtomicUsize`).
  
Note: The same statements also apply to strings (since strings are backed by the binary implementation).

**What operations are allocation-free / zero-copy?**

It's not documented (in text) - and of course depends on the implementation ... but for the default-implementation (`NewBin`/`NewSBin`/`NewStr`/`NewSStr`) there's a test, see [tests/no_alloc_guarantees.rs](tests/no_alloc_guarantees.rs).

**I want to write my own implementation, how to?**

There's currently no documentation - but you can use the default implementation for reference. It's found in the module `implementation`.

**Why `Boo` and not `Cow`?**

`Cow` requires `where B: 'a + ToOwned`. This does not work with this crate, since the implementation is separated from the interface. Say we have `&[u8]` (borrowed), to convert that to owned (`Bin` or `SBin`), the implementation has to be known. I don't want `Cow` to contain information about the implementation.

**Aren't `Bin` and `Str` huge (stack-size)?**

`Bin` and `Str` have a size of 4 words and are word-aligned. Yes, it's not small - but for reference, a `Vec<u8>` also takes 3 words (pointer, length and capacity).

**What is re-integration?**

Say we have this code (pseudocode):

```rust
let large_binary_from_network : Vec<u8> = <...>;
let bin = NewBin::from_given_vec(large_binary_from_network);
let slice_of_that_bin : &[u8] = &bin.as_slice()[45..458];

// it's now possible to re-integrate that `slice_of_that_bin` into the `bin` it was sliced from.
// re-integration converts the borrowed type `&[u8]` (`slice_of_that_bin`) into an owned
// type (`Bin`) without memory-allocation or memory-copy.
let bin_re_integrated : Bin = bin.try_re_integrate(slice_of_that_bin).unwrap();
```

This is useful if you want to de-serialize to owned (without using `Boo`) using serde. When deserializing a type, we get `slice_of_that_bin` from serde; using re-integration it's possible to get an owned binary (`Bin`) without allocation.

Technical detail: It checks whether `slice_of_that_bin` lies within the memory range of `bin`; if so, it increments the reference-count of `bin` by one, and the returned binary (`bin_re_integrated`) is then just a sliced reference to `bin`.

**Name `abin`?**

It's named after the trait `AnyBin`.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).