use crate::AnyBin;

pub enum BinSegment<'a, TAnyBin: AnyBin> {
    Slice(&'a [u8]),
    Static(&'static [u8]),
    Bin(TAnyBin),
    GivenVec(Vec<u8>),
    Empty,
}

impl<'a, TAnyBin> BinSegment<'a, TAnyBin>
where
    TAnyBin: AnyBin,
{
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        match self {
            BinSegment::Slice(slice) => *slice,
            BinSegment::Static(slice) => *slice,
            BinSegment::Bin(bin) => bin.as_slice(),
            BinSegment::GivenVec(vec) => vec.as_slice(),
            BinSegment::Empty => &[],
        }
    }

    pub fn from_slice(slice: &'a [u8]) -> Self {
        Self::Slice(slice)
    }
}

impl<'a, TAnyBin> From<&'static [u8]> for BinSegment<'a, TAnyBin>
where
    TAnyBin: AnyBin,
{
    fn from(slice: &'static [u8]) -> Self {
        Self::Static(slice)
    }
}

impl<'a, TAnyBin> From<TAnyBin> for BinSegment<'a, TAnyBin>
where
    TAnyBin: AnyBin,
{
    fn from(bin: TAnyBin) -> Self {
        Self::Bin(bin)
    }
}

impl<'a, TAnyBin> From<Vec<u8>> for BinSegment<'a, TAnyBin>
where
    TAnyBin: AnyBin,
{
    fn from(vec: Vec<u8>) -> Self {
        Self::GivenVec(vec)
    }
}
