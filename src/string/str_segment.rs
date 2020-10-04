use crate::{AnyBin, AnyStr, BinSegment, Segment};

pub enum StrSegment<'a, TBin: AnyBin> {
    Slice(&'a str),
    Static(&'static str),
    Str(AnyStr<TBin>),
    GivenString(String),
    Empty,
}

impl<'a, TBin: AnyBin> Into<BinSegment<'a, TBin>> for StrSegment<'a, TBin> {
    fn into(self) -> BinSegment<'a, TBin> {
        match self {
            StrSegment::Slice(slice) => BinSegment::Slice(slice.as_bytes()),
            StrSegment::Static(slice) => BinSegment::Static(slice.as_bytes()),
            StrSegment::Str(string) => BinSegment::Bin(string.into_bin()),
            StrSegment::GivenString(string) => BinSegment::GivenVec(string.into_bytes()),
            StrSegment::Empty => BinSegment::Empty,
        }
    }
}

impl<'a, TBin: AnyBin> From<&'static str> for StrSegment<'a, TBin> {
    fn from(string: &'static str) -> Self {
        Self::Static(string)
    }
}

impl<'a, TBin: AnyBin> From<AnyStr<TBin>> for StrSegment<'a, TBin> {
    fn from(any_str: AnyStr<TBin>) -> Self {
        Self::Str(any_str)
    }
}

impl<'a, TBin: AnyBin> From<String> for StrSegment<'a, TBin> {
    fn from(string: String) -> Self {
        Self::GivenString(string)
    }
}

impl<'a, TBin: AnyBin> Segment for StrSegment<'a, TBin> {
    #[inline]
    fn number_of_bytes(&self) -> usize {
        match self {
            StrSegment::Slice(slice) => slice.len(),
            StrSegment::Static(slice) => slice.len(),
            StrSegment::Str(string) => string.len(),
            StrSegment::GivenString(string) => string.len(),
            StrSegment::Empty => 0,
        }
    }

    #[inline]
    fn empty() -> Self {
        Self::Empty
    }
}
