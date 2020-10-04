use crate::{AnyBin, BinSegment, Segment};

/// An iterator for `Segment`.
///
/// Difference from a normal iterator (this is required for efficient binary construction):
///
///  * It knows exactly the number of bytes of all segments combined.
///  * It can tell whether there's just one single non-empty segment in this iterator.
pub trait SegmentIterator<TSegment>: IntoIterator<Item = TSegment>
where
    TSegment: Segment,
{
    /// Returns `true` if the exact number of bytes are known (or likely to be known). Note: The
    /// implementation uses this for optimization. The implementation MUST NOT fail if this
    /// returns a wrong value.
    ///
    /// Returns `None` if this value cannot be determined.
    fn exact_number_of_bytes(&self) -> Option<usize>;

    /// Returns `true` if this implementation knows that it's empty (has no segments or number of
    /// bytes is 0). This information MUST be correct. If the implementation is not sure or
    /// it's not empty: return `false`.
    fn is_empty(&self) -> bool;

    /// If this iterator contains exactly one (non-empty) segment, returns this single segment.
    /// Returns `Err` otherwise. (note: empty segments can be ignored; they don't count).
    fn single(self) -> Result<TSegment, Self>
    where
        Self: Sized;
}
