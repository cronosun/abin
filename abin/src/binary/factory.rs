use crate::{AnyBin, BinSegment, ExcessShrink, SegmentIterator};

/// Use this factory to create binaries. There's a built-in implementation in this crate;
/// custom implementations (that implement this trait) are possible.
pub trait BinFactory {
    /// The type this factory produces.
    type T: AnyBin;

    /// Empty binary.
    ///
    /// ```rust
    /// use abin::{NewBin, Bin, BinFactory};
    /// let bin : Bin = NewBin::empty();
    /// assert_eq!(0, bin.len());
    /// ```
    fn empty() -> Self::T;

    /// A binary from a `&'static [u8]`.
    ///
    /// ```rust
    /// use abin::{Bin, NewBin, BinFactory, AnyBin};
    /// let bin : Bin = NewBin::from_static("Hello".as_bytes());
    /// assert_eq!("Hello".as_bytes(), bin.as_slice());
    /// ```
    fn from_static(slice: &'static [u8]) -> Self::T;

    /// A binary from a `&[u8]`; prefer `from_static` if there's a static lifetime.
    ///
    /// ```rust
    /// use abin::{Bin, NewBin, BinFactory, AnyBin};
    /// let bin : Bin = NewBin::copy_from_slice("Hello".as_bytes());
    /// assert_eq!("Hello".as_bytes(), bin.as_slice());
    /// ```
    fn copy_from_slice(slice: &[u8]) -> Self::T;

    /// Create a binary from an iterator. To be efficient, the iterator should provide correct
    /// hints (see `Iterator::size_hint`).
    fn from_iter_with_config<T: GivenVecConfig, TIterator>(iter: TIterator) -> Self::T
    where
        TIterator: IntoIterator<Item = u8>;

    /// Create a binary from an iterator. To be efficient, the iterator should provide correct
    /// hints (see `Iterator::size_hint`).
    ///
    /// ```rust
    /// use abin::{Bin, NewBin, BinFactory, AnyBin};
    /// let bin : Bin = NewBin::from_iter((0..6).map(|i| i as u8));
    /// assert_eq!(&[0u8, 1u8, 2u8, 3u8, 4u8, 5u8], bin.as_slice());
    /// ```
    fn from_iter(iter: impl IntoIterator<Item = u8>) -> Self::T;

    /// Create a binary by joining multiple segments (see `BinSegment`).
    fn from_segments_with_config<'a, T: GivenVecConfig, TIterator>(iter: TIterator) -> Self::T
    where
        TIterator: SegmentIterator<BinSegment<'a, Self::T>>;

    /// Create a binary by joining multiple segments (see `BinSegment`).
    ///
    /// ```rust
    /// use abin::{BinSegment, SegmentsSlice, Bin, NewBin, BinFactory, AnyBin};
    ///
    /// let segments = &mut [BinSegment::Static("Hello, ".as_bytes()),
    ///     BinSegment::Static("World!".as_bytes())];
    /// let iterator = SegmentsSlice::new(segments);
    /// let bin : Bin = NewBin::from_segments(iterator);
    /// assert_eq!("Hello, World!".as_bytes(), bin.as_slice());
    /// ```
    fn from_segments<'a>(iter: impl SegmentIterator<BinSegment<'a, Self::T>>) -> Self::T;

    /// Convert a `BinSegment` to a binary.
    fn from_segment_with_config<'a, T: GivenVecConfig, TSegment>(segment: TSegment) -> Self::T
    where
        TSegment: Into<BinSegment<'a, Self::T>>;

    /// Convert a `BinSegment` to a binary.
    ///
    /// ```rust
    /// use abin::{NewBin, BinFactory, BinSegment};
    ///
    /// // both lines are equivalent (`from_segment` will just call `from_static`).
    /// let bin_1 = NewBin::from_static("Hello".as_bytes());
    /// let bin_2 = NewBin::from_segment(BinSegment::Static("Hello".as_bytes()));
    ///
    /// assert_eq!(bin_1, bin_2);
    /// ```
    fn from_segment<'a>(segment: impl Into<BinSegment<'a, Self::T>>) -> Self::T;

    /// Creates a binary from given vec. Important: Only use this method if you're given
    /// a `Vec<u8>` from outside (something you can't control). If you're in control, use
    /// any of the other methods provided by this factory (such as `from_iter`, `from_segments`).
    ///
    /// See `from_given_vec_with_config` with a default config chosen by the implementation.
    ///
    /// ```rust
    /// use abin::{NewBin, BinFactory, AnyBin};
    /// let vec_from_string = "Hello".to_owned().into_bytes();
    /// let bin = NewBin::from_given_vec(vec_from_string);
    /// assert_eq!("Hello".as_bytes(), bin.as_slice());
    /// ```
    fn from_given_vec(vec: Vec<u8>) -> Self::T;

    /// Creates a binary from given vec. Important: Only use this method if you're given
    /// a `Vec<u8>` from outside (something you can't control). If you're in control, use
    /// any of the other methods provided by this factory (such as `from_iter`, `from_segments`).
    fn from_given_vec_with_config<T: GivenVecConfig>(vec: Vec<u8>) -> Self::T;
}

/// Custom configuration used for `BinFactory::from_given_vec` /
/// `BinFactory::from_given_vec_with_config`).
pub trait GivenVecConfig {
    /// Shrink the given vector if there's too much excess?
    /// (excess = `Vec::capacity() - Vec::len()`).
    type TExcessShrink: ExcessShrink;
    /// Optimization hint.
    fn optimization() -> GivenVecOptimization;
}

/// Hint on what optimization to perform when constructing a binary from a `Vec<u8>`. Optimize
/// for construction (see `BinFactory::from_given_vec`) or optimize for operations
/// (such as `clone` or `slice`).
#[derive(Debug, Eq, PartialEq)]
pub enum GivenVecOptimization {
    /// Optimize for construction (when `AnyBin` is created from `Vec<u8>`). Operations (such as
    /// `clone` or `slice`) might be slower (require allocation / mem-copy).
    Construction,

    /// Optimize for operations (such as `clone`, `slice`). The creation (when `AnyBin` is created
    /// from `Vec<u8>`) might be slower (require allocation / mem-copy) instead.
    Operations,
}
