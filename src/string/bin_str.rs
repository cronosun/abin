use crate::{AnyStr, Bin, Factory, IntoSync, IntoUnSyncView, New, SStr};

/// A string backed by [Bin](struct.Bin.html) (not `Sync + Send`).
pub type Str = AnyStr<Bin>;
