use crate::{AnyBin, BinSegment};

pub trait SegmentsIterator<'a, TAnyBin>: IntoIterator<Item = BinSegment<'a, TAnyBin>>
where
    TAnyBin: AnyBin,
{
    type TAnyBin: AnyBin;

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

    /// If this iterator contains exactly one segment, returns this single segment. Returns
    /// `Err` otherwise. (note: empty segments can be ignored; they don't count).
    fn single(self) -> Result<BinSegment<'a, TAnyBin>, Self>
    where
        Self: Sized;
}
