// TODO
pub trait IntoSync {
    type Target;

    /// Converts this into a synchronized version.
    ///
    /// It's cheap if this is already backed by a synchronized implementation (or if it's just a
    /// view). See also `IntoUnSyncView` / `IntoUnSync`. If it's not backed by a synchronized
    /// implementation, this operation might be expensive: for instance if you apply this
    /// operation on a reference-counted binary that's not synchronized and has multiple
    /// references pointing to it, the data of the binary must be cloned.
    fn into_sync(self) -> Self::Target;
}
