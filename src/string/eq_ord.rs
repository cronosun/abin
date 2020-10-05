use std::cmp::Ordering;

use crate::{AnyBin, AnyStr};

impl<TBin> Eq for AnyStr<TBin> where TBin: AnyBin {}

impl<TBin> PartialEq for AnyStr<TBin>
where
    TBin: AnyBin,
{
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl<TBin> PartialEq<str> for AnyStr<TBin>
where
    TBin: AnyBin,
{
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
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

impl<TBin> PartialOrd<str> for AnyStr<TBin>
where
    TBin: AnyBin,
{
    fn partial_cmp(&self, other: &str) -> Option<Ordering> {
        self.as_str().partial_cmp(other)
    }
}
