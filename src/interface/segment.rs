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
}
