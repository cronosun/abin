use crate::{AnyBin, ExcessShrink};

pub trait Factory {
    type T: AnyBin;

    fn empty() -> Self::T;
    fn from_static(slice: &'static [u8]) -> Self::T;
    fn copy_from_slice(slice: &[u8]) -> Self::T;
    fn from_iter(iter: impl IntoIterator<Item=u8>) -> Self::T;
    fn vec_excess() -> usize;
    fn from_vec(vec: Vec<u8>) -> Self::T;
    fn from_vec_reduce_excess<T: ExcessShrink>(vec: Vec<u8>) -> Self::T;
}