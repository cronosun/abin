//! A utility library for working with binaries. It provides multiple implementations that all
//! share the same interface ([AnyBin](trait.AnyBin.html),
//! [struct Bin](struct.Bin.html)/[struct SyncBin](struct.SyncBin.html)). [Bin](struct.Bin.html)
//! and [SyncBin](struct.SyncBin.html) have no lifetime arguments, are sized (structs), easy
//! to use, most operations are allocation-free, and they can be converted to each other.
//! [SyncBin](struct.SyncBin.html) is a version of [Bin](struct.Bin.html) that
//! implements `Send + Sync`.
//!
//! The implementations are [EmptyBin](struct.EmptyBin.html), [RcBin](struct.RcBin.html),
//! [ArcBin](struct.ArcBin.html), [VecBin](struct.VecBin.html), [StackBin](struct.StackBin.html)
//! and [StaticBin](struct.StaticBin.html). Cutoms implementations are possible.
//!
//! To work with strings (utf-8 strings), there's [AnyStr](struct.AnyStr.html)
//! ([Str](type.Str.html) / [SyncStr](type.SyncStr.html)).
//!
//! ```rust
//! use abin::{AnyBin, AnyRc, ArcBin, Bin, EmptyBin, RcBin, StaticBin, VecBin};
//!
//! pub fn usage() {
//!     let example_slice = "This is some binary used for the following examples.".as_bytes();
//!
//!     // empty binary, stack-only.
//!     let bin1 = EmptyBin::new();
//!     // small binary; stack-only.
//!     let bin2 = RcBin::copy_from_slice(&example_slice[2..5]);
//!     // reference-counted binary (non-synchronized; like Rc);
//!     let bin3 = RcBin::copy_from_slice(example_slice);
//!     // reference-counted binary (synchronized; like Arc);
//!     let bin4 = ArcBin::from_vec(example_slice.to_vec());
//!     // binary backed by a Vec<u8>.
//!     let bin5 = VecBin::from_vec(example_slice.to_vec(), false);
//!     // no allocation for static data.
//!     let bin6 = StaticBin::from(example_slice);
//!
//!     use_bin(bin1.into(), &[]);
//!     use_bin(bin2, &example_slice[2..5]);
//!     use_bin(bin3, example_slice);
//!     use_bin(bin4.into(), example_slice);
//!     use_bin(bin5.into(), example_slice);
//!     use_bin(bin6.into(), example_slice);
//! }
//!
//! pub fn use_bin(bin: Bin, expected: &[u8]) {
//!     assert_eq!(expected.len(), bin.len());
//!     assert_eq!(expected, bin.as_slice());
//!     let cloned: Bin = bin.clone();
//!     assert_eq!(bin, cloned);
//!     let slice: Option<Bin> = bin.slice(1..3);
//!     if let Some(slice) = slice {
//!         assert_eq!(2, slice.len());
//!     }
//!     let vec_from_bin = bin.into_vec();
//!     assert_eq!(cloned.as_slice(), vec_from_bin.as_slice());
//! }
//! ```

pub use {::serde::*, binary::*, interface::*, string::*};

mod binary;
mod interface;
mod serde;
mod string;
