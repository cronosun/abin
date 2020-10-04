use std::str::Utf8Error;

use crate::{
    AnyStr, AnyStrUtf8Error, BinFactory, SegmentIterator, SegmentIteratorConverter, StrSegment,
};

pub trait StrFactory {
    type TBinFactory: BinFactory;

    #[inline]
    fn from_segments<'a>(
        iter: impl SegmentIterator<StrSegment<'a, <Self::TBinFactory as BinFactory>::T>>,
    ) -> AnyStr<<Self::TBinFactory as BinFactory>::T> {
        let converter = SegmentIteratorConverter::new(iter);
        let bin = Self::TBinFactory::from_segments(converter);
        // we know that it's valid (since each segment is valid; this should result in a valid utf-8 output).
        unsafe { AnyStr::from_utf8_unchecked(bin) }
    }

    #[inline]
    fn from_segment<'a>(
        segment: impl Into<StrSegment<'a, <Self::TBinFactory as BinFactory>::T>>,
    ) -> AnyStr<<Self::TBinFactory as BinFactory>::T> {
        let bin = Self::TBinFactory::from_segment(segment.into());
        // we know that it's valid utf-8.
        unsafe { AnyStr::from_utf8_unchecked(bin) }
    }

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

    #[inline]
    fn empty() -> AnyStr<<Self::TBinFactory as BinFactory>::T> {
        let bin = Self::TBinFactory::empty();
        // empty is always valid utf-8
        unsafe { AnyStr::from_utf8_unchecked(bin) }
    }

    #[inline]
    fn from_static(string: &'static str) -> AnyStr<<Self::TBinFactory as BinFactory>::T> {
        let bin = Self::TBinFactory::from_static(string.as_bytes());
        // we know it's valid utf-8
        unsafe { AnyStr::from_utf8_unchecked(bin) }
    }

    #[inline]
    fn from_given_string(string: impl Into<String>) -> AnyStr<<Self::TBinFactory as BinFactory>::T> {
        let string = string.into();
        let bin = Self::TBinFactory::from_given_vec(string.into_bytes());
        // we know it's valid utf-8
        unsafe { AnyStr::from_utf8_unchecked(bin) }
    }

    #[inline]
    fn copy_from_str<'a>(string: impl Into<&'a str>) -> AnyStr<<Self::TBinFactory as BinFactory>::T> {
        let string = string.into();
        let bin = Self::TBinFactory::copy_from_slice(string.as_bytes());
        // we know it's valid utf-8
        unsafe { AnyStr::from_utf8_unchecked(bin) }
    }
}
