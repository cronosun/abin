use crate::Bin;

/// The function table to be implemented for [Bin](struct.Bin.html) types.
///
/// Note: This is only required if you implement your own binary type.
pub struct FnTable {
    /// Drop function. It's `None` if dropping is not required.
    pub drop: Option<fn(bin: &mut Bin)>,

    /// Returns a slice of this binary.
    ///
    /// It's allowed to be `None` if this binary is always empty (constant).
    pub as_slice: Option<fn(bin: &Bin) -> &[u8]>,

    /// True if this binary has a length of 0.
    ///
    /// It's allowed to be `None` if this binary is always empty (constant).
    pub is_empty: Option<fn(bin: &Bin) -> bool>,

    /// Clones this type.
    ///
    /// IMPORTANT: It's required to return a sync binary if self is also a
    /// sync binary (this is not checked by the compiler, it's in the responsibility of the
    /// implementer).
    pub clone: fn(bin: &Bin) -> Bin,

    /// Converts this binary into a vector; Tries to avoid allocation/memory-copy whenever possible.
    pub into_vec: fn(bin: Bin) -> Vec<u8>,

    /// Returns a slice of the given binary. Returns `None` if the given range is out of bounds.
    ///
    /// Important: If `bin` is synchronized, the returned `Bin` MUST be synchronized too.
    pub slice: fn(bin: &Bin, start: usize, end_excluded: usize) -> Option<Bin>,

    /// Returns an un-synchronized version (not just a view - if there's an un-synchronized version).
    ///
    /// This is allowed to be `None` if:
    ///
    ///   * This is already the un-synchronized version.
    ///   * There's no un-synchronized version.
    pub convert_into_un_sync: Option<fn(bin: Bin) -> Bin>,

    /// Returns a synchronized version.
    ///
    /// This is allowed to be `None` if: This is already the synchronized version. IMPORTANT:
    /// IT IS NEVER allowed to return `None` here if this is not the synchronized version (this
    /// can't be checked by the compiler; it's in the responsibility of the implementer).
    pub convert_into_sync: Option<fn(bin: Bin) -> Bin>,

    /// Tries to re-integrate the given slice into the given binary.
    ///
    /// Details: If the given binary is a slice of the given binary, it returns a re-integrated
    /// version. Example: Say `bin` is a reference-counted binary from address 150 to 220
    /// (length 70) and the given slice points to memory address 170 and has a length of 30,
    /// this function returns a slice of the reference-counted binary (start 20, length 30).
    ///
    /// This is `None` if the binary type does not support re-integration altogether. This
    /// function returns `None` if the given slice cannot be re-integrated. This method usually
    /// makes only sense for reference-counted binaries or static binaries. This is purely an
    /// optimization - it's valid to always return `None` here.
    ///
    /// IMPORTANT: If `bin` is a synchronized binary, the returned binary has to be
    /// synchronized too.
    pub try_re_integrate: Option<fn(bin: &Bin, slice: &[u8]) -> Option<Bin>>,
}
