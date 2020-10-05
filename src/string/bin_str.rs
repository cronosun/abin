use crate::{AnyStr, Bin};

/// A string backed by `Bin` (not `Sync + Send`), see `SStr` if you need `Sync + Send`.
pub type Str = AnyStr<Bin>;
