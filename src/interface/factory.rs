use crate::{AnyBin, BinSegment, ExcessShrink, SegmentIterator};

pub trait Factory {
    /// The type this factory produces.
    type T: AnyBin;

    fn empty() -> Self::T;
    fn from_static(slice: &'static [u8]) -> Self::T;
    fn copy_from_slice(slice: &[u8]) -> Self::T;

    fn from_iter_with_config<T: GivenVecConfig, TIterator>(iter: TIterator) -> Self::T
    where
        TIterator: IntoIterator<Item = u8>;

    fn from_iter(iter: impl IntoIterator<Item = u8>) -> Self::T;

    fn from_segments_with_config<'a, T: GivenVecConfig, TIterator>(iter: TIterator) -> Self::T
    where
        TIterator: SegmentIterator<BinSegment<'a, Self::T>>;

    fn from_segments<'a>(iter: impl SegmentIterator<BinSegment<'a, Self::T>>) -> Self::T;

    fn from_segment_with_config<'a, T: GivenVecConfig, TSegment>(segment: TSegment) -> Self::T
    where
        TSegment: Into<BinSegment<'a, Self::T>>;

    fn from_segment<'a>(segment: impl Into<BinSegment<'a, Self::T>>) -> Self::T;

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
