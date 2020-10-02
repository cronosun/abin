use core::fmt;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Deref, RangeBounds};
use std::str::Utf8Error;

use crate::{AnyBin, StaticBin};

/// A utf-8 string backed by [AnyBin](trait.AnyBin.html) ([Bin](struct.Bin.html) or
/// [SyncBin](struct.SyncBin.html)), see also [Str](type.Str.html) and
/// [SyncStr](type.SyncStr.html).
pub struct AnyStr<TBin>(TBin);

impl<TBin> AnyStr<TBin>
// TODO implement: From<'static str> -> From<String> -> Self::from_str()
    where
        TBin: AnyBin,
{
    /// Converts the given value to a string.
    ///
    /// The given value must be valid UTF-8. If the value is not valid UTF-8, this method
    /// returns an error containing the original binary.
    #[inline]
    pub fn from_utf8(value: impl Into<TBin>) -> Result<Self, AnyStrUtf8Error<TBin>> {
        let value = value.into();
        // check whether it's valid UTF8
        if let Err(err) = core::str::from_utf8(value.as_slice()) {
            Err(AnyStrUtf8Error::new(err, value))
        } else {
            // ok, valid UTF8
            Ok(Self(value))
        }
    }

    #[inline]
    pub unsafe fn from_utf8_unchecked(value: TBin) -> Self {
        Self(value)
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        // no need to check utf8 again; we know it's valid (it's validated when constructed).
        unsafe { core::str::from_utf8_unchecked(self.0.as_slice()) }
    }

    #[inline]
    pub fn into_bin(self) -> TBin {
        self.0
    }
}

impl<TBin> Eq for AnyStr<TBin> where TBin: AnyBin {}

impl<TBin> PartialEq for AnyStr<TBin>
    where
        TBin: AnyBin,
{
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl<TBin> Ord for AnyStr<TBin>
    where
        TBin: AnyBin,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl<TBin> PartialOrd for AnyStr<TBin>
    where
        TBin: AnyBin,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl<TBin> Hash for AnyStr<TBin>
    where
        TBin: AnyBin,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_str().hash(state)
    }
}

impl<TBin> Debug for AnyStr<TBin>
    where
        TBin: AnyBin,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(self.as_str(), f)
    }
}

impl<TBin> Display for AnyStr<TBin>
    where
        TBin: AnyBin,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self.as_str(), f)
    }
}

impl<TBin> Into<String> for AnyStr<TBin>
    where
        TBin: AnyBin,
{
    #[inline]
    fn into(self) -> String {
        let vec = self.0.into_vec();
        // we already know it's valid UTF-8
        unsafe { String::from_utf8_unchecked(vec) }
    }
}

impl<TBin> Clone for AnyStr<TBin>
    where
        TBin: AnyBin,
{
    #[inline]
    fn clone(&self) -> Self {
        unsafe { AnyStr::from_utf8_unchecked(self.0.clone()) }
    }
}

impl<TBin> Deref for AnyStr<TBin>
    where
        TBin: AnyBin,
{
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<TBin> Borrow<str> for AnyStr<TBin>
    where
        TBin: AnyBin,
{
    #[inline]
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

pub struct AnyStrUtf8Error<TBin> {
    utf8_error: Utf8Error,
    binary: TBin,
}

impl<TBin> AnyStrUtf8Error<TBin> {
    pub fn new(utf8_error: Utf8Error, binary: TBin) -> Self {
        Self { utf8_error, binary }
    }

    pub fn utf8_error(&self) -> &Utf8Error {
        &self.utf8_error
    }

    pub fn deconstruct(self) -> (Utf8Error, TBin) {
        (self.utf8_error, self.binary)
    }
}
