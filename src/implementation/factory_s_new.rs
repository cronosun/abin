use std::marker::PhantomData;

use crate::{AnyRc, ArcBin, Bin, DefaultExcessShrink, EmptyBin, ExcessShrink, Factory, IntoUnSyncView, maybe_shrink, RcBin, SBin, StaticBin, VecBin, StackBin};

pub struct SNew {
    _phantom: PhantomData<()>
}
