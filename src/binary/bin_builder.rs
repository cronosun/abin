use crate::{AnyBin, BinSegment};

pub trait BinBuilder<'a> {
    type T: AnyBin;

    fn push(&mut self, segment: impl Into<BinSegment<'a, Self::T>>);

    /*fn push_bin(&mut self, bin: impl Into<Self::T>);
    fn push_slice(&mut self, slice: &[u8]);
    fn push_static(&mut self, slice: &'static [u8]);
    fn push_given_vec(&mut self, vec: Vec<u8>);
    fn push_u8(&mut self, byte: u8);*/

    /// Builds the binary.
    ///
    /// Note: After calling this method, the builder will be empty again and can be re-used. We
    /// use `&mut self` here instead of `self` to make sure the builder is not copied (it's large).
    /// I'm not sure how well rust would optimize `self` here.
    fn build(&mut self) -> Self::T;
}
