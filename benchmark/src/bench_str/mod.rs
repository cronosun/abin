mod abin_str;
mod arc_optimized;
mod arc_str_only;
mod bytes_str;
mod s_str;
mod string_only;

pub use {abin_str::*, arc_optimized::*, arc_str_only::*, bytes_str::*, s_str::*, string_only::*};

/// A string with multiple implementations; used for the benchmarks and the memory-test.
pub trait BenchStr: Clone {
    fn from_str(slice: &str) -> Self;
    fn from_static(slice: &'static str) -> Self;
    fn from_bin_iter(iter: impl Iterator<Item = u8>) -> Option<Self>
    where
        Self: Sized;
    fn from_multiple(iter: impl Iterator<Item = Self>) -> Self
    where
        Self: Sized;

    fn as_slice(&self) -> &str;
    fn slice(&self, start: usize, end: usize) -> Option<Self>
    where
        Self: Sized;
    fn into_string(self) -> String;
}
