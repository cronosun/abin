use std::ops::Deref;

pub trait AnyBin: Deref<Target=[u8]> + Clone /*+ Debug + Eq + PartialEq + Hash + Ord + PartialOrd*/ {
    /// Returns a view into this binary as slice.
    #[inline]
    fn as_slice(&self) -> &[u8] {
        self.deref()
    }

    /// Converts this binary into a `Vec<u8>` - the implementation tries to avoid copying memory
    /// whenever possible (best effort).
    fn into_vec(self) -> Vec<u8>;
}