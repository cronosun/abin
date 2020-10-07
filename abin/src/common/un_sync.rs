/// Returns the un-synchronized view of self.
///
/// See also `UnSyncRef` and `IntoUnSync`.
pub trait IntoUnSyncView {
    type Target;

    /// Returns the un-synchronized view of self.
    ///
    /// What does 'view' mean: The implementation is not really changed, just the interface is
    /// changed. It's still backed by a synchronized implementation.
    ///
    /// ```rust
    /// use abin::{SBin, NewSBin, BinFactory, IntoUnSyncView, Bin, AnyBin, IntoSync};
    /// let string = "This is some string; content of the binary.";
    /// let sync_bin : SBin = NewSBin::copy_from_slice(string.as_bytes());
    /// function_wants_bin(sync_bin.un_sync());
    ///
    /// fn function_wants_bin(value : Bin) {
    ///     // note: the 'value' here is still a synchronized binary (it just wrapped inside an
    ///     // un-synchronized view).
    ///     assert_eq!("This is some string; content of the binary.".as_bytes(), value.as_slice());
    ///     // we can also un-wrap it to be a synchronized bin again... in this case, this is
    ///     // a cheap operation (but it's not always a cheap operation).
    ///     let _synchronized_again : SBin = value.into_sync();
    /// }
    /// ```
    fn un_sync(self) -> Self::Target;
}

/// Returns the un-synchronized view of self (as reference).
///
/// See also `IntoUnSyncView` and `IntoUnSync`.
pub trait UnSyncRef {
    type Target;

    /// Returns the un-synchronized view of self (as reference).
    ///
    /// What does 'view' mean: The implementation is not really changed, just the interface is
    /// changed. It's still backed by a synchronized implementation.
    ///
    /// ```rust
    /// use abin::{NewSBin, SBin, BinFactory, UnSyncRef, Bin, AnyBin};
    /// let string = "This is some string; content of the binary.";
    /// let sync_bin : SBin = NewSBin::copy_from_slice(string.as_bytes());
    /// function_wants_bin(sync_bin.un_sync_ref());
    ///
    /// fn function_wants_bin(value : &Bin) {
    ///     // note: the 'value' here is still a synchronized binary (it just wrapped inside an
    ///     // un-synchronized view).
    ///     assert_eq!("This is some string; content of the binary.".as_bytes(), value.as_slice());
    /// }
    /// ```
    fn un_sync_ref(&self) -> &Self::Target;
}

/// Converts self into the un-synchronized version.
///
/// See also `IntoUnSyncView` and `UnSyncRef`.
pub trait IntoUnSync {
    type Target;

    /// Converts self into the un-synchronized version.
    ///
    /// Note: Unlike the `IntoUnSyncView` this does not always just return a view, it might
    /// actually change the backend (depending on the implementation). This operation
    /// might be expensive - depending on the implementation (for example a reference counted
    /// binary must clone its data if there are multiple references to that binary). So if
    /// there's no good reason to use this, better use the `IntoUnSyncView`.
    ///
    /// ```rust
    /// use abin::{NewSBin, SBin, BinFactory, Bin, IntoUnSync, AnyBin, IntoSync};
    /// let string = "This is some string; content of the binary.";
    /// let sync_bin : SBin = NewSBin::copy_from_slice(string.as_bytes());
    /// function_wants_bin(sync_bin.un_sync_convert());
    ///
    /// fn function_wants_bin(value : Bin) {
    ///     // note: The 'value' is no longer sync. E.g. the reference counter of this binary
    ///     // is no longer synchronized.
    ///     assert_eq!("This is some string; content of the binary.".as_bytes(), value.as_slice());
    ///     // we can also un-wrap it to be a synchronized bin again... in this case, this is
    ///     // a cheap operation (since there are no other references to `value`).
    ///     let _synchronized_again : SBin = value.into_sync();
    /// }
    /// ```
    fn un_sync_convert(self) -> Self::Target;
}
