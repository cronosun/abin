/// Returns the un-synchronized version of self.
pub trait UnSync {
    type Target;
    fn un_sync(self) -> Self::Target;
}

/// Returns the un-synchronized version of self (as reference).
pub trait UnSyncRef {
    type Target;
    fn un_sync_ref(&self) -> &Self::Target;
}
