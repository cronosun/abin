/// Common trait for the synchronized and the non-synchronized reference counted binary
/// ([RcBin](struct.RcBin.html) and [ArcBin](struct.ArcBin.html)).
pub trait AnyRc {
    /// The binary type produced.
    type T;

    /// This creates a reference counted binary; this involves copying the given slice.
    ///
    /// ```rust
    /// use abin::{RcBin, AnyRc, StaticBin, IntoUnSyncView};
    /// let slice = "Hello, world!".as_bytes();
    /// let bin1 = RcBin::copy_from_slice(slice);
    /// // note: this is what you would actually use in this case, since the string is static:
    /// let bin2 = StaticBin::from("Hello, world!".as_bytes());
    /// assert_eq!(bin1, bin2.un_sync());
    /// ```
    fn copy_from_slice(slice: &[u8]) -> Self::T;

    /// This creates a reference counted binary; It will first create a vector (with enough
    /// capacity for the meta-data) and then call `Self::from_vec`.
    ///
    /// Simple example:
    ///
    /// ```rust
    /// use abin::{RcBin, AnyRc, AnyBin};
    /// let iter = (0..5).map(|item| item*2 as u8);
    /// let bin = RcBin::from_iter(iter);
    /// assert_eq!(5, bin.len());
    /// assert_eq!(&[0, 2, 4, 6, 8], bin.as_slice());
    /// ```
    ///
    /// A more reallistic example:
    ///
    /// ```rust
    /// use abin::{StaticBin, EmptyBin, RcBin, AnyRc, SegmentsSlice, AnyBin};
    ///
    /// let bin1 = StaticBin::from("static value, ".as_bytes());
    /// let bin2 = RcBin::copy_from_slice("another binary".as_bytes());
    ///
    /// // the rest of the code needs just one single allocation (RcBin::from_iter allocates).
    /// let slice = [bin1.as_slice(), bin2.as_slice()];
    /// let chain_slices = SegmentsSlice::from(&slice as &[&[u8]]);
    /// let chained = RcBin::from_iter(chain_slices);
    /// assert_eq!("static value, another binary".as_bytes(), chained.as_slice());
    /// ```
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
    /// ```rust
    /// use abin::{AnyRc, AnyBin, StackBin, ArcBin};
    /// let item1 = "Hello, ".as_bytes();
    /// let item2 = "world! Bonjour! Hallo!".as_bytes();
    ///
    /// // this line is the important one: reserve additional capacity to prevent additional
    /// // allocation: + RcBin::overhead_bytes()
    /// let required_capacity = item1.len() + item2.len() + ArcBin::overhead_bytes();
    /// let mut vec = Vec::with_capacity(required_capacity);
    /// vec.extend_from_slice(item1);
    /// vec.extend_from_slice(item2);
    ///
    /// // this does not allocate, since the given vector has enough capacity for the meta-data.
    /// let bin = ArcBin::from_vec(vec);
    /// assert_eq!("Hello, world! Bonjour! Hallo!".as_bytes(), bin.as_slice());
    /// ```
    fn from_vec(vec: Vec<u8>) -> Self::T;

    /// This is the overhead required to store the reference count. It's typically about 7 bytes
    /// (32 bit reference count and up to 3 bytes of padding). Use this if you manually build a
    /// vector.
    ///
    /// ```rust
    /// use abin::{RcBin, AnyRc, AnyBin, StackBin};
    /// let item1 = "Hello, ".as_bytes();
    /// let item2 = "world! Bonjour! Hallo!".as_bytes();
    ///
    /// // this line is the important one: reserve additional capacity to prevent additional
    /// // allocation: + RcBin::overhead_bytes()
    /// let required_capacity = item1.len() + item2.len() + RcBin::overhead_bytes();
    /// let mut vec = Vec::with_capacity(required_capacity);
    /// vec.extend_from_slice(item1);
    /// vec.extend_from_slice(item2);
    /// let vector_address = vec.as_ptr();
    ///
    /// // this does not allocate, since the given vector has enough capacity for the meta-data.
    /// let bin = RcBin::from_vec(vec);
    /// assert_eq!("Hello, world! Bonjour! Hallo!".as_bytes(), bin.as_slice());
    ///
    /// // addition: just to make sure it's still the same allocation.
    /// // without this check, the test would fail on 128-bit machines.
    /// if bin.len()>StackBin::max_len() {
    ///   assert_eq!(vector_address, bin.into_vec().as_ptr());
    /// }
    /// ```
    fn overhead_bytes() -> usize;
}
