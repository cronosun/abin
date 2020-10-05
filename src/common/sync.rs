/// Converts this into a synchronized version.
///
/// See also `IntoUnSyncView`, `IntoUnSync` and `UnSyncRef`.
pub trait IntoSync {
    type Target;

    /// Converts this into a synchronized version.
    ///
    /// It's cheap if this is already backed by a synchronized implementation (or if it's just a
    /// view). See also `IntoUnSyncView` / `IntoUnSync`. If it's not
    /// backed by a synchronized implementation, this operation might be expensive: for instance
    /// if you apply this operation on a reference-counted binary that's not synchronized and has
    /// multiple references pointing to it, the data of the binary must be cloned.
    ///
    /// ```rust
    /// use abin::{NewBin, Bin, BinFactory, SBin, IntoSync, NewSBin, AnyBin};
    ///
    /// let string = "this is the content of this binary";
    /// let not_sync : Bin = NewBin::copy_from_slice(string.as_bytes());
    /// // this line 'converts' (not just a view) the binary into a sync binary (after that call
    /// // the reference-counter is synchronized).
    /// let sync_1 : SBin = not_sync.into_sync();
    /// // this is the direct way to construct a synchronized binary.
    /// // sync_1 and sync_2 are equivalent.
    /// let sync_2 : SBin = NewSBin::copy_from_slice(string.as_bytes());
    /// assert_eq!(string.as_bytes(), sync_1.as_slice());
    /// assert_eq!(sync_1, sync_2);
    /// ```
    fn into_sync(self) -> Self::Target;
}
