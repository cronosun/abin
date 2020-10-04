use std::marker::PhantomData;

use crate::{AnyRc, Bin, BinBuilder, BuilderCfg, DefaultBinBuilder, IntoUnSyncView, RcBin, SBin};

pub struct NewBin {
    _phantom: PhantomData<()>,
}

impl NewBin {
    pub fn builder<'a>() -> impl BinBuilder<'a, T = Bin> {
        DefaultBinBuilder::<NewBin, BinBuilderCfg>::new()
    }
}

struct BinBuilderCfg;

impl BuilderCfg<Bin> for BinBuilderCfg {
    fn convert_from_sbin_to_t(sbin: SBin) -> Bin {
        sbin.un_sync()
    }

    fn vec_excess_capacity() -> usize {
        RcBin::overhead_bytes()
    }
}
