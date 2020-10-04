use crate::AnyBin;

pub trait BinBuilder {
    type T: AnyBin;

    fn push_bin(&mut self, bin: impl Into<Self::T>);
    fn push_slice(&mut self, slice: &[u8]);
    fn push_static(&mut self, slice: &'static [u8]);
    fn push_given_vec(&mut self, vec: Vec<u8>);
    fn push_u8(&mut self, byte: u8);

    /// Builds the binary. Note: After calling this method, the builder will be empty again.
    fn build(&mut self) -> Self::T;
}
