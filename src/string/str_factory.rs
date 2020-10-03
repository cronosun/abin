use crate::{AnyStr, Factory, New, SNew, SStr, Str};

pub trait StrFactory {
    type TBinFactory: Factory;

    #[inline]
    fn from_static(string: &'static str) -> AnyStr<<Self::TBinFactory as Factory>::T> {
        let bin = Self::TBinFactory::from_static(string.as_bytes());
        // we know it's valid utf-8
        unsafe { AnyStr::from_utf8_unchecked(bin) }
    }

    #[inline]
    fn from_string(string: impl Into<String>) -> AnyStr<<Self::TBinFactory as Factory>::T> {
        let string = string.into();
        let bin = Self::TBinFactory::from_given_vec(string.into_bytes());
        // we know it's valid utf-8
        unsafe { AnyStr::from_utf8_unchecked(bin) }
    }

    #[inline]
    fn copy_from_str<'a>(string: impl Into<&'a str>) -> AnyStr<<Self::TBinFactory as Factory>::T> {
        let string = string.into();
        let bin = Self::TBinFactory::copy_from_slice(string.as_bytes());
        // we know it's valid utf-8
        unsafe { AnyStr::from_utf8_unchecked(bin) }
    }
}
