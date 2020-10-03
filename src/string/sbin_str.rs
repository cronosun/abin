use crate::{AnyStr, SBin};

/// A string backed by [SyncBin](struct.SyncBin.html) (`Sync + Send`).
pub type SStr = AnyStr<SBin>;
