use crate::{
    AnyStr, AnyStrUtf8Error, BinFactory, SegmentIterator, SegmentIteratorConverter, StrSegment,
};

/// The result produced by `from_utf8_iter`. Is either a `AnyStr` or an `AnyStrUtf8Error` on
/// error (invalid UTF-8).
pub type FromUtf8IterResult<TAnyBin> = Result<AnyStr<TAnyBin>, AnyStrUtf8Error<TAnyBin>>;

/// Use this factory to create strings. There's a built-in implementation in this crate;
/// custom implementations (that implement this trait) are possible.
pub trait StrFactory {
    /// The binary backend this string factory uses to produce strings (strings are just wrappers
    /// for utf-8-validated binaries).
    type TBinFactory: BinFactory;

    /// Create a string by joining multiple segments (see `StrSegment`).
    ///
    /// ```rust
    /// use abin::{StrSegment, SegmentsSlice, Str, NewStr, StrFactory, StrBuilder};
    /// let segments = &mut [StrSegment::Static("Hello, "),
    ///     StrSegment::Static("World!")];
    /// let iterator = SegmentsSlice::new(segments);
    /// let str : Str = NewStr::from_segments(iterator);
    /// assert_eq!("Hello, World!", str.as_str());
    /// ```
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
    ///
    /// ```rust
    /// use abin::{StrSegment, NewStr, StrFactory};
    ///
    /// // both lines are equivalent (`from_segment` will just call `from_static`).
    /// let str_1 = NewStr::from_static("Hello");
    /// let str_2 = NewStr::from_segment(StrSegment::Static("Hello"));
    ///
    /// assert_eq!(str_1, str_2);
    /// ```
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
    ///
    /// ```rust
    /// use abin::{NewStr, Str, StrFactory};
    /// let str : Str = NewStr::from_utf8_iter((65..72).map(|i| i as u8)).unwrap();
    /// assert_eq!("ABCDEFG", str.as_str());
    /// ```
    #[inline]
    fn from_utf8_iter(
        iter: impl IntoIterator<Item = u8>,
    ) -> FromUtf8IterResult<<Self::TBinFactory as BinFactory>::T> {
        let bin: <Self::TBinFactory as BinFactory>::T = Self::TBinFactory::from_iter(iter);
        AnyStr::<<Self::TBinFactory as BinFactory>::T>::from_utf8(bin)
    }

    /// Empty string.
    ///
    /// ```rust
    /// use abin::{NewStr, StrFactory, Str};
    /// let str : Str = NewStr::empty();
    /// assert_eq!(0, str.len());
    /// ```
    #[inline]
    fn empty() -> AnyStr<<Self::TBinFactory as BinFactory>::T> {
        let bin = Self::TBinFactory::empty();
        // empty is always valid utf-8
        unsafe { AnyStr::from_utf8_unchecked(bin) }
    }

    /// A string from a `&'static str`.
    ///
    /// ```rust
    /// use abin::{Str, NewStr, StrFactory};
    /// let str : Str = NewStr::from_static("Hello");
    /// assert_eq!("Hello", str.as_str());
    /// ```
    #[inline]
    fn from_static(string: &'static str) -> AnyStr<<Self::TBinFactory as BinFactory>::T> {
        let bin = Self::TBinFactory::from_static(string.as_bytes());
        // we know it's valid utf-8
        unsafe { AnyStr::from_utf8_unchecked(bin) }
    }

    /// Creates a string from given `String`. Important: Only use this method if you're given
    /// a `String` from outside (something you can't control). If you're in control, use
    /// any of the other methods provided by this factory (such as `from_iter`, `from_segments`).
    ///
    /// ```rust
    /// use abin::{Str, NewStr, StrFactory};
    /// let some_string : String = "This is some text".to_owned();
    /// let str : Str = NewStr::from_given_string(some_string);
    /// assert_eq!("This is some text", str.as_str());
    /// ```
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
    ///
    /// ```rust
    /// use abin::{Str, NewStr, StrFactory};
    /// let str : Str = NewStr::copy_from_str("Hello");
    /// assert_eq!("Hello", str.as_str());
    /// ```
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
