/// Some sort of segment that knows its length (in bytes). It's used for constructing
/// binaries/strings efficiently (knowing the entire length in advance to avoid
/// re-allocations).
pub trait Segment {
    /// The number of bytes in this segment.
    fn number_of_bytes(&self) -> usize;

    /// Same as `number_of_bytes==0`.
    #[inline]
    fn is_empty(&self) -> bool {
        self.number_of_bytes() == 0
    }

    /// Constructs an empty-segment (`number_of_bytes` is 0).
    fn empty() -> Self;
}
