use crate::{AnyBin, BinSegment, Bytes128};

/// Trait used to build a binary efficiently (with just one allocation & no re-allocation or
/// even without allocation).
///
/// ```rust
/// use abin::{NewBin, BinSegment, Bin, AnyBin, BinBuilder};
///
/// let mut builder = NewBin::builder();
/// builder.push(BinSegment::Static("Hello, ".as_bytes()));
/// builder.push(BinSegment::Static("World!".as_bytes()));
/// let bin : Bin = builder.build();
///
/// assert_eq!("Hello, World!".as_bytes(), bin.as_slice());
/// ```
pub trait BinBuilder<'a> {
    type T: AnyBin;

    fn push(&mut self, segment: impl Into<BinSegment<'a, Self::T>>);

    #[inline]
    fn push_bin(&mut self, bin: impl Into<Self::T>) {
        self.push(BinSegment::Bin(bin.into()));
    }

    #[inline]
    fn push_slice(&mut self, slice: impl Into<&'a [u8]>) {
        self.push(BinSegment::Slice(slice.into()));
    }

    #[inline]
    fn push_static(&mut self, slice: impl Into<&'static [u8]>) {
        self.push(BinSegment::Static(slice.into()));
    }

    #[inline]
    fn push_given_vec(&mut self, vec: impl Into<Vec<u8>>) {
        self.push(BinSegment::GivenVec(vec.into()));
    }

    #[inline]
    fn push_u8(&mut self, byte: u8) {
        let bin_segment: Bytes128 = byte.into();
        self.push(BinSegment::Bytes128(bin_segment));
    }

    /// Builds the binary.
    ///
    /// Note: After calling this method, the builder will be empty again and can be re-used. We
    /// use `&mut self` here instead of `self` to make sure the builder is not copied (it's large).
    /// I'm not sure how well rust would optimize `self` here.
    fn build(&mut self) -> Self::T;
}
