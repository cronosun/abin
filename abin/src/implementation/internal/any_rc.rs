/// Common trait for the synchronized and the non-synchronized reference counted binary.
pub trait AnyRc {
    /// The binary type produced.
    type T;

    /// This creates a reference counted binary; this involves copying the given slice.
    fn copy_from_slice(slice: &[u8]) -> Self::T;

    /// This creates a reference counted binary; It will first create a vector (with enough
    /// capacity for the meta-data) and then call `Self::from_vec`.
    fn from_iter(iter: impl IntoIterator<Item = u8>) -> Self::T;

    /// Creates a reference counted binary from a vector while trying to avoid memory allocation &
    /// memory copy (best effort).
    ///
    /// Important: Make sure the given `Vec<u8>` has at least an excess (capacity - len) of
    /// `Self::overhead_bytes`. If not, the binary can be created, but this function might allocate
    /// memory and also some operations might need to allocate later (like `to_vec`, `into_sync`).
    ///
    ///  * Note 1: The given vector is used to store the reference count - this avoids (in
    /// most cases) another indirection and memory copy (best effort).
    ///
    ///  * Note 2: Do not manually shrink the given vector, the implementation needs the additional
    /// capacity to store the reference count. The implementation will shrink the vector itself,
    /// see `Self::from_with_cap_shrink` if you need to configure this behaviour.
    ///
    ///  * Note 3: If you manually create the vector using `Vec::with_capacity` make sure you
    /// reserve additional space for the reference count to avoid a re-allocation.
    /// See `Self::overhead_bytes`;
    ///
    fn from_vec(vec: Vec<u8>) -> Self::T;

    /// This is the overhead required to store the reference count. It's typically about 7 bytes
    /// (32 bit reference count and up to 3 bytes of padding). Use this if you manually build a
    /// vector.
    fn overhead_bytes() -> usize;
}
