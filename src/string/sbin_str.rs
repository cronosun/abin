use crate::{AnyRc, AnyStr, ArcBin, IntoUnSyncView, SBin, StaticBin, Str};

/// A string backed by [SyncBin](struct.SyncBin.html) (`Sync + Send`).
pub type SStr = AnyStr<SBin>;
