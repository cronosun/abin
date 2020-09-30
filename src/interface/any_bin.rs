use std::borrow::Borrow;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::RangeBounds;

pub trait AnyBin: Clone + Debug + Eq + PartialEq + Hash + Ord + PartialOrd + Borrow<[u8]> {
    /// Returns slice-view into this binary.
    fn as_slice(&self) -> &[u8];

    /// Converts this binary into a `Vec<u8>` - the implementation tries to avoid copying memory
    /// whenever possible (best effort).
    fn into_vec(self) -> Vec<u8>;

    /// The length (number of bytes).
    fn len(&self) -> usize;

    /// Returns a slice if the given range is within bounds.
    ///
    /// Returns `None` if the range is out of bounds (otherwise the implementation is required
    /// to return `Some`). Tries to avoid allocations / memory copy whenever possible (best
    /// effort).
    fn slice<TRange>(&self, range: TRange) -> Option<Self>
    where
        TRange: RangeBounds<usize>;
}
