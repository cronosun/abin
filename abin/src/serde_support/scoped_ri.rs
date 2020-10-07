use core::cell::RefCell;
use core::mem;
use std::{marker::PhantomData, ops::Deref};

use crate::{Bin, SBin};

std::thread_local! {
  static THREAD_LOCAL_BIN: RefCell<Option<*const ScopedRiSetup<'static>>> = RefCell::new(None);
}

/// This struct contains functions that give access to the re-integration scope.
pub struct RiScope {
    // pure static
    _phantom: PhantomData<()>,
}

impl RiScope {
    /// Returns the re-integrated binary. See `AnyBin::try_to_re_integrate`.
    ///
    /// This returns `None` if:
    ///
    ///   * There's no RI scope set up.
    ///   * `ReIntegrationFn` returned `None`.
    #[inline]
    pub fn try_re_integrate(slice: &[u8]) -> Option<Bin> {
        THREAD_LOCAL_BIN.with(|value| {
            let borrowed = value.borrow();
            if let Some(value) = borrowed.deref() {
                let config: &ScopedRiSetup = unsafe { &**value };
                (config.re_integration_fn)(&config.binaries, slice)
            } else {
                None
            }
        })
    }

    /// Returns the re-integrated binary. See `AnyBin::try_to_re_integrate`.
    ///
    /// This returns `None` if:
    ///
    ///   * There's no RI scope set up.
    ///   * `SyncReIntegrationSync` returned `None`.
    #[inline]
    pub fn try_re_integrate_sync(slice: &[u8]) -> Option<SBin> {
        THREAD_LOCAL_BIN.with(|value| {
            let borrowed = value.borrow();
            if let Some(value) = borrowed.deref() {
                let config: &ScopedRiSetup = unsafe { &**value };
                (config.sync_re_integration_fn)(&config.binaries, slice)
            } else {
                None
            }
        })
    }
}

pub type ReIntegrationFn = fn(binaries: &Binaries, slice: &[u8]) -> Option<Bin>;
pub type SyncReIntegrationSync = fn(binaries: &Binaries, slice: &[u8]) -> Option<SBin>;

/// A re-integration scope setup.
pub struct ScopedRiSetup<'a> {
    binaries: Binaries<'a>,
    re_integration_fn: ReIntegrationFn,
    sync_re_integration_fn: SyncReIntegrationSync,
}

impl<'a> ScopedRiSetup<'a> {
    pub fn new(
        binaries: Binaries<'a>,
        re_integration_fn: ReIntegrationFn,
        sync_re_integration_fn: SyncReIntegrationSync,
    ) -> Self {
        Self {
            binaries,
            re_integration_fn,
            sync_re_integration_fn,
        }
    }
}

impl<'a> ScopedRiSetup<'a> {
    #[inline]
    pub fn scoped<TFn, TRet>(&self, fun: TFn) -> TRet
    where
        TFn: FnOnce() -> TRet,
    {
        THREAD_LOCAL_BIN.with(|tl_value| {
            // save the previous value (in case we nest scopes).
            let previous_value = tl_value.borrow_mut().take();

            // make sure the thread-local-value is removed even if the function panics (this
            // is important, since we messed with lifetimes -> 'a to 'static ... so we have
            // to make sure it is NEVER accesses outside the scope). This is dropped at the
            // end of this scope.
            let cleanup_on_drop = CleanupOnDrop {
                cell: tl_value,
                previous_value,
            };

            // new thread-local-value
            let this = unsafe {
                mem::transmute::<&ScopedRiSetup<'a>, *const ScopedRiSetup<'static>>(self)
            };
            tl_value.replace(Some(this));

            let result = fun();

            // i'm not 100% sure whether the rust specification allows optimization of unused
            // values (e.g eager dropping; dropping before scope ends)... but to make sure
            // it does not do this, we use it here (it's a no-op).
            cleanup_on_drop.done();

            result
        })
    }
}

pub struct Binaries<'a> {
    bin: Option<&'a Bin>,
    sync_bin: Option<&'a SBin>,
}

impl<'a> Binaries<'a> {
    pub fn new(bin: Option<&'a Bin>, sync_bin: Option<&'a SBin>) -> Self {
        Self { bin, sync_bin }
    }

    pub fn new_bin(bin: &'a Bin) -> Self {
        Self::new(Some(bin), None)
    }

    pub fn new_sync_bin(bin: &'a SBin) -> Self {
        Self::new(None, Some(bin))
    }

    #[inline]
    pub fn bin(&self) -> Option<&'a Bin> {
        self.bin
    }

    #[inline]
    pub fn sync_bin(&self) -> Option<&'a SBin> {
        self.sync_bin
    }

    #[inline]
    pub fn both(&self) -> (Option<&'a Bin>, Option<&'a SBin>) {
        (self.bin, self.sync_bin)
    }
}

/// This is required to cleanup to scope in case the scope-function panics.
struct CleanupOnDrop<'a> {
    cell: &'a RefCell<Option<*const ScopedRiSetup<'static>>>,
    previous_value: Option<*const ScopedRiSetup<'static>>,
}

impl<'a> CleanupOnDrop<'a> {
    #[inline]
    fn done(&self) {
        // intentionally a no-op
    }
}

impl<'a> Drop for CleanupOnDrop<'a> {
    fn drop(&mut self) {
        // sets the previous value
        self.cell.replace(self.previous_value.take());
    }
}
