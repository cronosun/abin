use std::marker::PhantomData;

use crate::{
    maybe_shrink, AnyRc, Bin, CommonFactory, DefaultExcessShrink, EmptyBin, ExcessShrink, Factory,
    IntoUnSyncView, RcBin, SBin, StackBin, StaticBin, SyncToUnSyncConverter, VecBin,
};

pub struct New {
    _phantom: PhantomData<()>,
}
