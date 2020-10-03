use crate::{AnyStr, Bin};

/// A string backed by [Bin](struct.Bin.html) (not `Sync + Send`).
pub type Str = AnyStr<Bin>;
