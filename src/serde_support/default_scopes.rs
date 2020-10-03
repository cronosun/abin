use serde::export::PhantomData;

use crate::{AnyBin, Bin, Binaries, IntoUnSyncView, SBin, ScopedRiSetup};

/// Constructs [ScopedRiSetup] with sane defaults.
pub struct DefaultScopes {
    _phantom: PhantomData<()>,
}

impl DefaultScopes {
    /// A scope that can be used to de-serialize `Bin` and `SyncBin`. If a `Bin` is required,
    /// the `SyncBin` value is converted using `IntoUnSyncView` (so just a view).
    ///
    /// This is what you most likely want to use if you can't guarantee that you don't
    /// de-serialize only `Bin`.
    pub fn sync(bin: &SBin) -> ScopedRiSetup {
        ScopedRiSetup::new(
            Binaries::new_sync_bin(bin),
            sync_re_integration_fn,
            sync_sync_re_integration_fn,
        )
    }
}

fn sync_re_integration_fn(binaries: &Binaries, slice: &[u8]) -> Option<Bin> {
    match binaries.both() {
        (Some(bin), _) => bin.try_to_re_integrate(slice),
        (_, Some(sync)) => sync.try_to_re_integrate(slice).map(|item| item.un_sync()),
        _ => None,
    }
}

fn sync_sync_re_integration_fn(binaries: &Binaries, slice: &[u8]) -> Option<SBin> {
    if let Some(bin) = binaries.sync_bin() {
        bin.try_to_re_integrate(slice)
    } else {
        None
    }
}
