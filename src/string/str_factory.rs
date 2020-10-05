use crate::{
    AnyStr, AnyStrUtf8Error, BinFactory, SegmentIterator, SegmentIteratorConverter, StrSegment,
};

/// Use this factory to create strings. There's a built-in implementation in this crate;
/// custom implementations (that implement this trait) are possible.
pub trait StrFactory {
    type TBinFactory: BinFactory;

    /// Create a string by joining multiple segments (see `StrSegment`).
    #[inline]
    fn from_segments<'a>(
        iter: impl SegmentIterator<StrSegment<'a, <Self::TBinFactory as BinFactory>::T>>,
    ) -> AnyStr<<Self::TBinFactory as BinFactory>::T> {
        let converter = SegmentIteratorConverter::new(iter);
        let bin = Self::TBinFactory::from_segments(converter);
        // we know that it's valid (since each segment is valid; this should result in a valid utf-8 output).
        unsafe { AnyStr::from_utf8_unchecked(bin) }
    }

    /// Convert a `StrSegment` to a string.
    #[inline]
    fn from_segment<'a>(
        segment: impl Into<StrSegment<'a, <Self::TBinFactory as BinFactory>::T>>,
    ) -> AnyStr<<Self::TBinFactory as BinFactory>::T> {
        let bin = Self::TBinFactory::from_segment(segment.into());
        // we know that it's valid utf-8.
        unsafe { AnyStr::from_utf8_unchecked(bin) }
    }

    /// Create a string from an iterator. To be efficient, the iterator should provide correct
    /// hints (see `Iterator::size_hint`).
    #[inline]
    fn from_utf8_iter(
        iter: impl IntoIterator<Item = u8>,
    ) -> Result<
        AnyStr<<Self::TBinFactory as BinFactory>::T>,
        AnyStrUtf8Error<<Self::TBinFactory as BinFactory>::T>,
    > {
        let bin: <Self::TBinFactory as BinFactory>::T = Self::TBinFactory::from_iter(iter);
        AnyStr::<<Self::TBinFactory as BinFactory>::T>::from_utf8(bin)
    }

    /// Empty string.
    #[inline]
    fn empty() -> AnyStr<<Self::TBinFactory as BinFactory>::T> {
        let bin = Self::TBinFactory::empty();
        // empty is always valid utf-8
        unsafe { AnyStr::from_utf8_unchecked(bin) }
    }

    /// A string from a `&'static str`.
    #[inline]
    fn from_static(string: &'static str) -> AnyStr<<Self::TBinFactory as BinFactory>::T> {
        let bin = Self::TBinFactory::from_static(string.as_bytes());
        // we know it's valid utf-8
        unsafe { AnyStr::from_utf8_unchecked(bin) }
    }

    /// Creates a string from given `String`. Important: Only use this method if you're given
    /// a `String` from outside (something you can't control). If you're in control, use
    /// any of the other methods provided by this factory (such as `from_iter`, `from_segments`).
    #[inline]
    fn from_given_string(
        string: impl Into<String>,
    ) -> AnyStr<<Self::TBinFactory as BinFactory>::T> {
        let string = string.into();
        let bin = Self::TBinFactory::from_given_vec(string.into_bytes());
        // we know it's valid utf-8
        unsafe { AnyStr::from_utf8_unchecked(bin) }
    }

    /// A string from a `&str`; prefer `from_static` if there's a static lifetime.
    #[inline]
    fn copy_from_str<'a>(
        string: impl Into<&'a str>,
    ) -> AnyStr<<Self::TBinFactory as BinFactory>::T> {
        let string = string.into();
        let bin = Self::TBinFactory::copy_from_slice(string.as_bytes());
        // we know it's valid utf-8
        unsafe { AnyStr::from_utf8_unchecked(bin) }
    }
}
