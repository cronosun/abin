use crate::Boo;
use std::borrow::Borrow;
use std::cmp::Ordering;

impl<'a, TBorrowed, TOwned> Eq for Boo<'a, TBorrowed, TOwned>
where
    TOwned: Borrow<TBorrowed>,
    TBorrowed: Eq + PartialEq,
    TBorrowed: ?Sized,
{
}

impl<'a, TBorrowed, TOwned> PartialEq for Boo<'a, TBorrowed, TOwned>
where
    TOwned: Borrow<TBorrowed>,
    TBorrowed: PartialEq,
    TBorrowed: ?Sized,
{
    fn eq(&self, other: &Self) -> bool {
        self.borrow_internal() == other.borrow_internal()
    }
}

impl<'a, TBorrowed, TOwned> Ord for Boo<'a, TBorrowed, TOwned>
where
    TOwned: Borrow<TBorrowed>,
    TBorrowed: Ord + Eq + PartialOrd,
    TBorrowed: ?Sized,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.borrow_internal().cmp(other.borrow_internal())
    }
}

impl<'a, TBorrowed, TOwned> PartialOrd for Boo<'a, TBorrowed, TOwned>
where
    TOwned: Borrow<TBorrowed>,
    TBorrowed: PartialOrd + PartialEq,
    TBorrowed: ?Sized,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.borrow_internal().partial_cmp(other.borrow_internal())
    }
}

impl<'a, TBorrowed, TOwned> PartialEq<TBorrowed> for Boo<'a, TBorrowed, TOwned>
where
    TOwned: Borrow<TBorrowed>,
    TBorrowed: PartialEq,
    TBorrowed: ?Sized,
{
    fn eq(&self, other: &TBorrowed) -> bool {
        self.borrow_internal() == other
    }
}

impl<'a, TBorrowed, TOwned> PartialOrd<TBorrowed> for Boo<'a, TBorrowed, TOwned>
where
    TOwned: Borrow<TBorrowed>,
    TBorrowed: PartialOrd + PartialEq,
    TBorrowed: ?Sized,
{
    fn partial_cmp(&self, other: &TBorrowed) -> Option<Ordering> {
        self.borrow_internal().partial_cmp(other)
    }
}
