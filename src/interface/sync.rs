/// Converts this into a synchronized version.
///
/// See also [IntoUnSyncView](trait.IntoUnSyncView.html), [IntoUnSync](trait.IntoUnSync.html)
/// and [UnSyncRef](trait.UnSyncRef.html).
pub trait IntoSync {
    type Target;

    /// Converts this into a synchronized version.
    ///
    /// It's cheap if this is already backed by a synchronized implementation (or if it's just a
    /// view). See also [IntoUnSyncView](trait.IntoUnSyncView.html) /
    /// [IntoUnSync](trait.IntoUnSync.html). If it's not
    /// backed by a synchronized implementation, this operation might be expensive: for instance
    /// if you apply this operation on a reference-counted binary that's not synchronized and has
    /// multiple references pointing to it, the data of the binary must be cloned.
    ///
    /// ```rust
    /// use abin::{RcBin, AnyRc, Bin, SyncBin, IntoSync, AnyBin, ArcBin};
    ///
    /// let string = "this is the content of this binary";
    /// let not_sync : Bin = RcBin::copy_from_slice(string.as_bytes());
    /// // this line 'converts' (not just a view) the binary into a sync binary (after that call
    /// // the reference-counter is synchronized).
    /// let sync_1 : SyncBin = not_sync.into_sync();
    /// // this is the direct way to construct a synchronized binary.
    /// // sync_1 and sync_2 are equivalent.
    /// let sync_2 : SyncBin = ArcBin::copy_from_slice(string.as_bytes());
    /// assert_eq!(string.as_bytes(), sync_1.as_slice());
    /// assert_eq!(sync_1, sync_2);
    /// ```
    fn into_sync(self) -> Self::Target;
}
