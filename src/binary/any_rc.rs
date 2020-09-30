use crate::VecCapShrink;

/// Common trait for the synchronized and the non-synchronized reference counted binary.
pub trait AnyRc {
    type T;

    /// Creates a reference counted binary from a vector while trying to avoid memory allocation &
    /// memory copy (best effort).
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
    /// See `Self::overhead_bytes`.
    fn from_vec(vec: Vec<u8>) -> Self::T;

    /// This creates a reference counted binary; this involves copying the given slice.
    fn copy_from_slice(slice: &[u8]) -> Self::T;

    /// This is the overhead required to store the reference count. It's typically about 7 bytes
    /// (32 bit reference count and up to 3 bytes of padding).
    fn overhead_bytes() -> usize;

    /// This is the same as `Self::from` but allows custom configuration whether to shrink
    /// the given vector.
    fn from_with_cap_shrink<T: VecCapShrink>(vec: Vec<u8>) -> Self::T;
}
