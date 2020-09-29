use crate::Bin;

/// The function table to be implemented for `Bin`.
///
/// Note: This is only required if you implement your own binary type.
pub struct FnTable {
    pub drop: fn(bin: &mut Bin),
    pub as_slice: fn(bin: &Bin) -> &[u8],
    pub is_empty: fn(bin: &Bin) -> bool,
    /// Clones this type.
    ///
    /// IMPORTANT: It's required to return a sync binary if self is also a
    /// sync binary (this is not checked by the compiler, it's in the responsibility of the
    /// implementer).
    pub clone: fn(bin: &Bin) -> Bin,
    pub into_vec: fn(bin: Bin) -> Vec<u8>,
    /// Returns a slice of the given binary. Returns `None` if the given range is out of bounds.
    pub slice: fn(bin: &Bin, start: usize, end_excluded: usize) -> Option<Bin>,
}