use crate::{AnyBin, Segment};

/// A segment; segments can be joined to create binaries. See `BinBuilder`,
/// `SegmentIterator` and `SegmentsSlice`.
#[derive(Debug, Clone)]
pub enum BinSegment<'a, TAnyBin: AnyBin> {
    Slice(&'a [u8]),
    Static(&'static [u8]),
    Bin(TAnyBin),
    GivenVec(Vec<u8>),
    Bytes128(Bytes128),
    Empty,
}

impl<'a, TAnyBin> Segment for BinSegment<'a, TAnyBin>
where
    TAnyBin: AnyBin,
{
    #[inline]
    fn number_of_bytes(&self) -> usize {
        self.as_slice().len()
    }

    #[inline]
    fn empty() -> Self {
        Self::Empty
    }
}

impl<'a, TAnyBin> BinSegment<'a, TAnyBin>
where
    TAnyBin: AnyBin,
{
    pub fn as_slice(&self) -> &[u8] {
        match self {
            BinSegment::Slice(slice) => *slice,
            BinSegment::Static(slice) => *slice,
            BinSegment::Bin(bin) => bin.as_slice(),
            BinSegment::GivenVec(vec) => vec.as_slice(),
            BinSegment::Bytes128(bytes) => bytes.as_slice(),
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

impl<'a, TAnyBin> From<Bytes128> for BinSegment<'a, TAnyBin>
where
    TAnyBin: AnyBin,
{
    fn from(bytes: Bytes128) -> Self {
        Self::Bytes128(bytes)
    }
}

const BYTES_128_LEN: usize = 16;

/// Up to 16 bytes / 128 bit stored on the stack.
///
/// This can be used when constructing segments from primitive types (such as char, u64, f32
/// or f64).
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Bytes128 {
    bytes: [u8; BYTES_128_LEN],
    len: u8,
}

impl Bytes128 {
    /// creates new bytes from given slice. Returns `None` if given slice contains more than
    /// `max_number_of_bytes` bytes.
    #[inline]
    pub fn try_new(slice: &[u8]) -> Option<Self> {
        let len = slice.len();
        if len > BYTES_128_LEN {
            None
        } else {
            let mut this = Self {
                bytes: [0u8; BYTES_128_LEN],
                len: len as u8,
            };
            let bytes = &mut this.bytes[0..len];
            bytes.copy_from_slice(slice);
            Some(this)
        }
    }

    /// The maximum number of bytes that can be stored.
    pub const fn max_number_of_bytes() -> usize {
        BYTES_128_LEN
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        &self.bytes[0..self.len as usize]
    }
}

impl From<u8> for Bytes128 {
    fn from(byte: u8) -> Self {
        Self {
            bytes: [byte, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            len: 1,
        }
    }
}

impl From<char> for Bytes128 {
    fn from(chr: char) -> Self {
        let mut buf = [0u8; 4];
        let string = chr.encode_utf8(&mut buf);
        Self::try_new(string.as_bytes()).expect(
            "Implementation error: Converting from \
        char cannot fail (since char takes 4 bytes whereas Bytes128 can take up to 16 bytes).",
        )
    }
}
