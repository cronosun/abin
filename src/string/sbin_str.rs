use crate::{AnyStr, SBin};

/// A string backed by `SBin` (`Sync + Send`), see `Str` if you don't need `Sync + Send`.
pub type SStr = AnyStr<SBin>;
