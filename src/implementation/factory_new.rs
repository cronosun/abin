use std::marker::PhantomData;

use crate::{AnyRc, Bin, CommonFactory, DefaultExcessShrink, EmptyBin, ExcessShrink, Factory, IntoUnSyncView, maybe_shrink, RcBin, SBin, StackBin, StaticBin, SyncToUnSyncConverter, VecBin};

pub struct New {
    _phantom: PhantomData<()>
}
