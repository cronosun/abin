use std::borrow::Borrow;
use std::fmt::{Debug, LowerHex, UpperHex};
use std::hash::Hash;
use std::ops::{Deref, RangeBounds};

use crate::{Bin, IntoSync, IntoUnSync, IntoUnSyncView, SBin, UnSyncRef};

/// Common trait implemented by [Bin](struct.Bin.html) and [SyncBin](struct.SyncBin.html).
pub trait AnyBin:
    Clone
    + Debug
    + Eq
    + PartialEq
    + Hash
    + Ord
    + PartialOrd
    + Borrow<[u8]>
    + AsRef<[u8]>
    + IntoIterator<Item = u8>
    + LowerHex
    + UpperHex
    + Into<Vec<u8>>
    + UnSyncRef<Target = Bin>
    + IntoUnSyncView<Target = Bin>
    + IntoUnSync<Target = Bin>
    + IntoSync<Target = SBin>
    + Deref<Target = [u8]>
{
    /// Returns a view into this binary.
    fn as_slice(&self) -> &[u8];

    /// Converts this binary into a `Vec<u8>` - the implementation tries to avoid copying memory
    /// whenever possible (best effort).
    fn into_vec(self) -> Vec<u8>;

    /// The length (number of bytes).
    ///
    /// ```rust
    /// use abin::{StaticBin, AnyBin};
    /// let bin1 = StaticBin::from("Hello".as_bytes());
    /// assert_eq!(5, bin1.len());
    /// ```
    fn len(&self) -> usize;

    /// `true` if this binary is empty.
    ///
    /// ```rust
    /// use abin::{StaticBin, AnyBin};
    /// let bin1 = StaticBin::from("".as_bytes());
    /// assert_eq!(true, bin1.is_empty());
    /// ```
    fn is_empty(&self) -> bool;

    /// Returns a slice if the given range is within bounds.
    ///
    /// Returns `None` if the range is out of bounds (otherwise the implementation is required
    /// to return `Some`). Tries to avoid allocations / memory copy whenever possible (best
    /// effort).
    ///
    /// ```rust
    /// use abin::{StaticBin, AnyBin, EmptyBin};
    ///
    /// let bin1 = StaticBin::from("This is some text!".as_bytes());
    /// assert_eq!("is some".as_bytes(), bin1.slice(5..12).unwrap().as_slice());
    /// assert_eq!("This is some text!".as_bytes(), bin1.slice(0..18).unwrap().as_slice());
    ///
    /// // out of bounds
    /// assert_eq!(None, EmptyBin::new().slice(0..1));
    /// assert_eq!(None, EmptyBin::new().slice(800..0));
    /// assert_eq!(None, bin1.slice(0..19));
    /// ```
    fn slice<TRange>(&self, range: TRange) -> Option<Self>
    where
        TRange: RangeBounds<usize>;

    /// Tries to re-integrate the given slice into `self`. To some extent (not 100%), this is the
    /// reverse of `as_slice`.
    ///
    /// Details: If the given binary is a slice of `self`, it returns a re-integrated
    /// version. Example: Say `self` is a reference-counted binary from memory-address 150 to 220
    /// (length 70) and the given slice points to memory address 170 and has a length of 30,
    /// this function returns a slice of the reference-counted binary (start 20, length 30).
    ///
    /// This is `None` if the binary type does not support re-integration altogether. This
    /// is `None` if the given slice cannot be re-integrated (for example if the given slice is
    /// completely unrelated to `self` - is not within the managed memory of `self`). This method
    /// makes sense for reference-counted binaries or static binaries. This is purely an
    /// optimization - it's valid to always return `None` here.
    ///
    /// Use case: Say you got some `Vec<u8>` from the network, convert that to `RcBin` (A) and
    /// then use that binary (A) to de-serialize some data (`Bin::as_slice`) using serde: When
    /// de-serializing a `Bin` (B), this `Bin` (B) could then re-integrate itself into bin (A) and
    /// thus prevent a memory-allocation; `Bin` (B) is then just a slice of `Bin` (A).
    ///
    /// ```rust
    /// use abin::{StaticBin, AnyBin};
    ///
    /// let bin_a_slice = "this is some static binary".as_bytes();
    /// let bin_a = StaticBin::from(bin_a_slice);
    ///
    /// let bin_b_slice = &bin_a.as_slice()[5..];
    ///
    /// // note: This does not allocate
    /// let bin_b = bin_a.try_to_re_integrate(bin_b_slice).unwrap();
    /// assert_eq!(bin_b.as_slice(), bin_b_slice);
    ///
    /// let bin_c_completely_unrelated_slice = "Something completely unrelated".as_bytes();
    /// assert_eq!(None, bin_a.try_to_re_integrate(bin_c_completely_unrelated_slice));
    /// ```
    fn try_to_re_integrate(&self, slice: &[u8]) -> Option<Self>;
}
