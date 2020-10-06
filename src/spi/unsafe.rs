use crate::SBin;
use crate::spi::{BinData, FnTable};

/// Unsafe interface for `Bin`. This is only to be used if you want to
/// implement your own binary type.
///
/// Note: Naming (the `_`) is a bit awkward: This is intentional: This trait is implemented
/// for `Bin` and we want to make sure the IDE does not auto-complete; or the user uses these
/// functions accidentally.
pub unsafe trait UnsafeBin {
    /// New binary from given data with given function table.
    ///
    /// # Safety
    ///
    /// This is unsafe. Use this only if you implement your own binary type and you know what
    /// you're doing. See the default-implementation for details.
    unsafe fn _new(data: BinData, fn_table: &'static FnTable) -> Self;

    /// A reference to the binary data.
    ///
    /// # Safety
    ///
    /// This is unsafe. Use this only if you implement your own binary type and you know what
    /// you're doing. See the default-implementation for details.
    unsafe fn _data(&self) -> &BinData;

    /// A mutable reference to the binary data.
    ///
    /// # Safety
    ///
    /// This is unsafe. Use this only if you implement your own binary type and you know what
    /// you're doing. See the default-implementation for details.
    unsafe fn _data_mut(&mut self) -> &mut BinData;

    /// Gets the function-table.
    ///
    /// # Safety
    ///
    /// This is unsafe. Use this only if you implement your own binary type and you know what
    /// you're doing. See the default-implementation for details.
    unsafe fn _fn_table(&self) -> &'static FnTable;

    /// Wraps this binary in a sync-binary. This will just be a view (it does not synchronize). So
    /// be sure the actual implementation (the function table) is really `Send + Sync`.
    ///
    /// # Safety
    ///
    /// This is unsafe. Use this only if you implement your own binary type and you know what
    /// you're doing. See the default-implementation for details.
    unsafe fn _into_sync(self) -> SBin;
}
