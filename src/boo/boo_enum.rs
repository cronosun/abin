use core::fmt;
use std::borrow::Borrow;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::Deref;

use crate::BooToOwned;

/// A borrowed-or-owned type (boo).
///
/// Similar to `std::borrow::Cow` but without the requirement of
/// `TBorrowed : ToOwned<Owned=TOwned>`.
#[derive(Debug)]
pub enum Boo<'a, TBorrowed, TOwned>
where
    TBorrowed: ?Sized,
{
    Borrowed(&'a TBorrowed),
    Owned(TOwned),
}

impl<'a, TBorrowed, TOwned> From<&'a TBorrowed> for Boo<'a, TBorrowed, TOwned>
where
    TBorrowed: ?Sized,
{
    fn from(borrowed: &'a TBorrowed) -> Self {
        Self::Borrowed(borrowed)
    }
}

pub trait ToBooConverter {
    fn borrowed<TBorrowed>(&self) -> Boo<TBorrowed, Self>
    where
        TBorrowed: ?Sized,
        Self: Borrow<TBorrowed>,
        Self: Sized;
    fn owned<'a, TBorrowed>(self) -> Boo<'a, TBorrowed, Self>
    where
        TBorrowed: ?Sized,
        Self: Sized;
}

impl<T> ToBooConverter for T {
    fn borrowed<TBorrowed>(&self) -> Boo<TBorrowed, Self>
    where
        TBorrowed: ?Sized,
        Self: Borrow<TBorrowed>,
        Self: Sized,
    {
        Boo::Borrowed(self.borrow())
    }

    fn owned<'a, TBorrowed>(self) -> Boo<'a, TBorrowed, Self>
    where
        TBorrowed: ?Sized,
        Self: Sized,
    {
        Boo::Owned(self)
    }
}

impl<'a, TBorrowed, TOwned> Boo<'a, TBorrowed, TOwned>
where
    TOwned: Borrow<TBorrowed>,
    TBorrowed: ?Sized,
{
    pub fn into_owned_with<TBooToOwned>(self) -> TOwned
    where
        TBooToOwned: BooToOwned<TBorrowed, TOwned>,
    {
        match self {
            Boo::Borrowed(borrowed) => TBooToOwned::convert_to_owned(borrowed),
            Boo::Owned(owned) => owned,
        }
    }
}

impl<'a, TBorrowed, TOwned> Boo<'a, TBorrowed, TOwned>
where
    TOwned: Borrow<TBorrowed>,
    TBorrowed: ?Sized,
{
    pub(crate) fn borrow_internal(&self) -> &TBorrowed {
        match self {
            Boo::Borrowed(value) => *value,
            Boo::Owned(value) => value.borrow(),
        }
    }
}

impl<'a, TBorrowed, TOwned> Boo<'a, TBorrowed, TOwned>
where
    TBorrowed: ToOwned<Owned = TOwned>,
    TOwned: Borrow<TBorrowed>,
    TBorrowed: ?Sized,
{
    pub fn into_owned(self) -> TOwned {
        match self {
            Boo::Borrowed(borrowed) => borrowed.to_owned(),
            Boo::Owned(owned) => owned,
        }
    }
}

impl<'a, TBorrowed, TOwned> Default for Boo<'a, TBorrowed, TOwned>
where
    TOwned: Default,
    TBorrowed: ?Sized,
{
    fn default() -> Self {
        Boo::Owned(TOwned::default())
    }
}

impl<'a, TBorrowed, TOwned> Borrow<TBorrowed> for Boo<'a, TBorrowed, TOwned>
where
    TOwned: Borrow<TBorrowed>,
    TBorrowed: ?Sized,
{
    fn borrow(&self) -> &TBorrowed {
        match self {
            Boo::Borrowed(value) => *value,
            Boo::Owned(value) => value.borrow(),
        }
    }
}

impl<'a, TBorrowed, TOwned> AsRef<TBorrowed> for Boo<'a, TBorrowed, TOwned>
where
    TOwned: AsRef<TBorrowed>,
    TBorrowed: ?Sized,
{
    fn as_ref(&self) -> &TBorrowed {
        match self {
            Boo::Borrowed(value) => *value,
            Boo::Owned(value) => value.as_ref(),
        }
    }
}

impl<'a, TBorrowed, TOwned> Deref for Boo<'a, TBorrowed, TOwned>
where
    TOwned: Deref<Target = TBorrowed>,
    TBorrowed: ?Sized,
{
    type Target = TBorrowed;

    fn deref(&self) -> &Self::Target {
        match self {
            Boo::Borrowed(value) => *value,
            Boo::Owned(value) => value.borrow(),
        }
    }
}

impl<'a, TBorrowed, TOwned> Hash for Boo<'a, TBorrowed, TOwned>
where
    TOwned: Borrow<TBorrowed>,
    TBorrowed: Hash,
    TBorrowed: ?Sized,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.borrow_internal().hash(state)
    }
}

impl<'a, TBorrowed, TOwned> Display for Boo<'a, TBorrowed, TOwned>
where
    TOwned: Borrow<TBorrowed>,
    TBorrowed: Display,
    TBorrowed: ?Sized,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self.borrow_internal(), f)
    }
}

impl<'a, TBorrowed, TOwned> Clone for Boo<'a, TBorrowed, TOwned>
where
    TOwned: Clone,
    TBorrowed: ?Sized,
{
    fn clone(&self) -> Self {
        match self {
            Boo::Borrowed(value) => Boo::Borrowed(value),
            Boo::Owned(value) => Boo::Owned(value.clone()),
        }
    }
}
