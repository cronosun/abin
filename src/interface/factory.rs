use crate::{AnyBin, BinSegment, ExcessShrink, SegmentsIterator};

pub trait Factory {
    /// The type this factory produces.
    type T: AnyBin;

    fn empty() -> Self::T;
    fn from_static(slice: &'static [u8]) -> Self::T;
    fn copy_from_slice(slice: &[u8]) -> Self::T;

    // TODO: Denke das kann weg (from_iter_new)
    fn from_iter(iter: impl IntoIterator<Item=u8>) -> Self::T;

    // TODO: Rename ('new')
    fn from_iter_new_with_config<'a, T: GivenVecConfig, TIterator>(
        iter: TIterator,
    ) -> Self::T where TIterator: SegmentsIterator<'a, Self::T>;

    // TODO: Rename ('new')
    fn from_iter_new<'a, TIterator>(
        iter: TIterator,
    ) -> Self::T where TIterator: SegmentsIterator<'a, Self::T>;

    fn from_segment_with_config<T: GivenVecConfig>(segment: BinSegment<Self::T>) -> Self::T;

    fn from_segment(segment: BinSegment<Self::T>) -> Self::T;

    #[deprecated(note = "Exposes internal details; not required when builder is added")]
    fn vec_excess() -> usize;

    /// Creates a binary from given vec. Important: Only use this method if you're given
    /// a `Vec<u8>` from outside (something you can't control). If you're in control, use
    /// any of the other methods provided by this factory.
    ///
    /// See `from_given_vec_with_config` with a default config chosen by the implementation.
    fn from_given_vec(vec: Vec<u8>) -> Self::T;

    /// Creates a binary from given vec. Important: Only use this method if you're given
    /// a `Vec<u8>` from outside (something you can't control). If you're in control, use
    /// any of the other methods provided by this factory.
    fn from_given_vec_with_config<T: GivenVecConfig>(vec: Vec<u8>) -> Self::T;
}

pub trait GivenVecConfig {
    type TExcessShrink: ExcessShrink;
    fn optimization() -> GivenVecOptimization;
}

#[derive(Debug, Eq, PartialEq)]
pub enum GivenVecOptimization {
    /// Optimize for creation (when `AnyBin` is created from `Vec<u8>`). Operations (such as
    /// `clone` or `slice`) might be slower (require allocation / mem-copy).
    Creation,

    /// Optimize for operations (such as `clone`, `slice`). The creation (when `AnyBin` is created
    /// from `Vec<u8>`) might be slower (require allocation / mem-copy) instead.
    Operations,
}
