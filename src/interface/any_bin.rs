use std::borrow::Borrow;
use std::fmt::{Debug, LowerHex, UpperHex};
use std::hash::Hash;
use std::ops::RangeBounds;

/// Common trait implemented by `Bin` and `SyncBin`.
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
}
