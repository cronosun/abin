use core::fmt;
use std::borrow::Borrow;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Bound, Deref, RangeBounds};
use std::str::Utf8Error;

use crate::{AnyBin, Bin, IntoSync, IntoUnSync, IntoUnSyncView, SBin};

/// A utf-8 string backed by `AnyBin` (`Bin` or `SBin`), see also `Str` and `SStr`.
pub struct AnyStr<TBin>(TBin);

impl<TBin> AnyStr<TBin>
where
    TBin: AnyBin,
{
    /// Converts the given value to a string.
    ///
    /// The given value must be valid UTF-8. If the value is not valid UTF-8, this method
    /// returns an error containing the original binary.
    ///
    /// See also: `core::str::from_utf8` / `std::string::String::from_utf8`.
    ///
    /// ```rust
    /// use abin::{NewBin, BinFactory, AnyStr, Bin};
    /// let bin : Bin = NewBin::from_static(&[65u8, 66u8, 67u8]);
    /// let str : AnyStr<Bin> = AnyStr::from_utf8(bin).unwrap();
    /// assert_eq!("ABC", str.as_str());
    /// ```
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

    /// Creates a new string from given binary without checking whether the data in the given
    /// binary is valid UTF-8.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it does not check that the bytes passed to it are valid
    /// UTF-8. If this constraint is violated, undefined behavior results, as `AnyStr`
    /// assumes that it only contains valid UTF-8. See also `core::str::from_utf8_unchecked`.
    #[inline]
    pub unsafe fn from_utf8_unchecked(value: TBin) -> Self {
        Self(value)
    }

    /// Returns `&str`. It's a cheap operation. See `AnyBin::as_slice`.
    ///
    /// ```rust
    /// use abin::{Str, NewStr, StrFactory};
    /// let string : Str = NewStr::from_static("Hello");
    /// assert_eq!("Hello", string.as_str());
    /// ```
    #[inline]
    pub fn as_str(&self) -> &str {
        // no need to check utf8 again; we know it's valid (it's validated when constructed).
        unsafe { core::str::from_utf8_unchecked(self.0.as_slice()) }
    }

    /// Extracts the wrapped binary.
    ///
    /// ```rust
    /// use abin::{Str, NewStr, StrFactory, AnyBin, Bin};
    /// let string : Str = NewStr::from_static("ABC");
    /// let bin : Bin = string.into_bin();
    /// assert_eq!(&[65u8, 66u8, 67u8], bin.as_slice());
    /// ```
    #[inline]
    pub fn into_bin(self) -> TBin {
        self.0
    }

    /// Wrapped binary as reference.
    ///
    /// ```rust
    /// use abin::{Str, NewStr, StrFactory, AnyBin, Bin};
    /// let string : Str = NewStr::from_static("ABC");
    /// let bin : &Bin = string.as_bin();
    /// assert_eq!(&[65u8, 66u8, 67u8], bin.as_slice());
    /// ```
    #[inline]
    pub fn as_bin(&self) -> &TBin {
        &self.0
    }

    /// `true` if this string is empty (number of utf-8 bytes is 0). See also `AnyBin::is_empty`.
    ///
    /// ```rust
    /// use abin::{Str, NewStr, StrFactory};
    /// let string : Str = NewStr::from_static("");
    /// assert_eq!(true, string.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// The number of utf-8 bytes in this string. See also `AnyBin::len`.
    ///
    /// ```rust
    /// use abin::{Str, NewStr, StrFactory};
    /// let string : Str = NewStr::from_static("Hello");
    /// assert_eq!(5, string.len());
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Converts this into a `String`; depending on the type, this might allocate or not. See
    /// also `AnyBin::into_vec`.
    ///
    /// ```rust
    /// use abin::{Str, NewStr, StrFactory};
    /// let string : Str = NewStr::from_static("Hello");
    /// let std_string : String = string.into_string();
    /// assert_eq!("Hello", &std_string);
    /// ```
    #[inline]
    pub fn into_string(self) -> String {
        let vec = self.0.into_vec();
        unsafe { String::from_utf8_unchecked(vec) }
    }

    /// Returns a slice of this string.
    ///
    /// Returns `None` if:
    ///
    ///   - range is ouf of bounds.
    ///   - or if the range does not lie on UTF-8 boundaries (see also `str::get`).
    ///
    /// See also `AnyBin::slice`.
    ///
    /// ```rust
    /// use abin::{NewStr, StrFactory, Str};
    /// let str : Str = NewStr::from_static("🗻∈🌏");
    ///
    /// // works
    /// let slice1 : Option<Str> = str.slice(0..4);
    /// assert_eq!(Some(NewStr::from_static("🗻")), slice1);
    ///
    /// // indices not on UTF-8 sequence boundaries
    /// let slice2 : Option<Str> = str.slice(1..);
    /// assert!(slice2.is_none());
    /// let slice3 : Option<Str> = str.slice(..8);
    /// assert!(slice3.is_none());
    /// // out of bounds
    /// let slice4 : Option<Str> = str.slice(..42);
    /// assert!(slice4.is_none());
    /// ```
    #[inline]
    pub fn slice<TRange>(&self, range: TRange) -> Option<Self>
    where
        TRange: RangeBounds<usize>,
    {
        let start = match range.start_bound() {
            Bound::Included(start) => *start,
            Bound::Excluded(start) => *start + 1,
            Bound::Unbounded => 0,
        };
        let end_excluded = match range.end_bound() {
            Bound::Included(end) => *end - 1,
            Bound::Excluded(end) => *end,
            Bound::Unbounded => self.len(),
        };
        // use str::get to check whether we're within range and lie on UTF-8 boundaries
        if self.as_str().get(start..end_excluded).is_some() {
            // ok
            let sliced_bin = self.as_bin().slice(start..end_excluded);
            if let Some(sliced_bin) = sliced_bin {
                // we know it's valid UTF-8 (confirmed by `str::get`).
                Some(unsafe { Self::from_utf8_unchecked(sliced_bin) })
            } else {
                None
            }
        } else {
            None
        }
    }
}

/// See `AnyStr::into_string`.
impl<TBin> Into<String> for AnyStr<TBin>
where
    TBin: AnyBin,
{
    #[inline]
    fn into(self) -> String {
        self.into_string()
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

impl<TBin> Clone for AnyStr<TBin>
where
    TBin: AnyBin,
{
    #[inline]
    fn clone(&self) -> Self {
        unsafe { AnyStr::from_utf8_unchecked(self.0.clone()) }
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

impl<TBin> AsRef<str> for AnyStr<TBin>
where
    TBin: AnyBin,
{
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<TBin> AsRef<TBin> for AnyStr<TBin>
where
    TBin: AnyBin,
{
    #[inline]
    fn as_ref(&self) -> &TBin {
        self.as_bin()
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

impl<TBin> IntoUnSyncView for AnyStr<TBin>
where
    TBin: AnyBin,
{
    type Target = AnyStr<Bin>;

    fn un_sync(self) -> Self::Target {
        unsafe { Self::Target::from_utf8_unchecked(self.0.un_sync()) }
    }
}

impl<TBin> IntoUnSync for AnyStr<TBin>
where
    TBin: AnyBin,
{
    type Target = AnyStr<Bin>;

    fn un_sync_convert(self) -> Self::Target {
        unsafe { Self::Target::from_utf8_unchecked(self.0.un_sync_convert()) }
    }
}

impl<TBin> IntoSync for AnyStr<TBin>
where
    TBin: AnyBin,
{
    type Target = AnyStr<SBin>;

    fn into_sync(self) -> Self::Target {
        let bin = self.0;
        let sync_bin = bin.into_sync();
        unsafe { Self::Target::from_utf8_unchecked(sync_bin) }
    }
}

/// Error returned when trying to create a `AnyStr` from a binary that contains invalid utf-8.
#[derive(Debug, Clone)]
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

    /// This can be used to retrieve the binary that has been used to construct the string.
    pub fn deconstruct(self) -> (Utf8Error, TBin) {
        (self.utf8_error, self.binary)
    }
}

impl<TBin> Error for AnyStrUtf8Error<TBin> where TBin: AnyBin {}

impl<TBin> Display for AnyStrUtf8Error<TBin>
where
    TBin: AnyBin,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.utf8_error, f)
    }
}
