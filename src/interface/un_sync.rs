pub trait IntoUnSyncView {
    type Target;

    /// Returns the un-synchronized view of self.
    ///
    /// What does 'view' mean: The implementation is not really changed, just the interface is
    /// changed. It's still backed by a synchronized implementation.
    fn un_sync(self) -> Self::Target;
}

pub trait UnSyncRef {
    type Target;

    /// Returns the un-synchronized view of self (as reference).
    ///
    /// What does 'view' mean: The implementation is not really changed, just the interface is
    /// changed. It's still backed by a synchronized implementation.
    fn un_sync_ref(&self) -> &Self::Target;
}

// TODO: Und brauchen wir das wiklich irgendwo?
pub trait IntoUnSync {
    type Target;

    /// Returns the un-synchronized version of self.
    ///
    /// Note: Unlike the `IntoUnSyncView` this does not always just return a view, it might
    /// actually change the backend (depending on the implementation). This operation
    /// might be expensive - depending on the implementation (for example a reference counted
    /// binary must clone its data if there are multiple references to that binary). So if
    /// there's no good reason to use this, better use the `IntoUnSyncView`.
    fn un_sync_convert(self) -> Self::Target;
}
