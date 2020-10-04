use crate::{AnyStr, Factory, AnyStrUtf8Error};
use std::str::Utf8Error;

pub trait StrFactory {
    type TBinFactory: Factory;

    #[inline]
    fn from_utf8_iter(iter: impl IntoIterator<Item = u8>) -> Result<AnyStr<<Self::TBinFactory as Factory>::T>, AnyStrUtf8Error<<Self::TBinFactory as Factory>::T>> {
        let bin: <Self::TBinFactory as Factory>::T = Self::TBinFactory::from_iter(iter);
        AnyStr::<<Self::TBinFactory as Factory>::T>::from_utf8(bin)
    }

    #[inline]
    fn empty() -> AnyStr<<Self::TBinFactory as Factory>::T> {
        let bin = Self::TBinFactory::empty();
        // empty is always valid utf-8
        unsafe { AnyStr::from_utf8_unchecked(bin) }
    }

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
