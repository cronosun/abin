use std::marker::PhantomData;

use crate::{
    maybe_shrink, AnyRc, ArcBin, Bin, DefaultExcessShrink, EmptyBin, ExcessShrink, Factory,
    IntoUnSyncView, RcBin, SBin, StackBin, StaticBin, VecBin,
};

pub struct SNew {
    _phantom: PhantomData<()>,
}
