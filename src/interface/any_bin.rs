pub trait AnyBin: Clone /*+ Debug + Eq + PartialEq + Hash + Ord + PartialOrd*/ {
    /// Returns a view into this binary as slice.
    fn as_slice(&self) -> &[u8];

    /// Converts this binary into a `Vec<u8>` - the implementation tries to avoid copying memory
    /// whenever possible (best effort).
    fn into_vec(self) -> Vec<u8>;

    /// The length (number of bytes).
    fn len(&self) -> usize;
}