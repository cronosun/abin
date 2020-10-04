use std::marker::PhantomData;

use crate::{AnyRc, ArcBin, BinBuilder, BuilderCfg, DefaultBinBuilder, SBin};

pub struct NewSBin {
    _phantom: PhantomData<()>,
}

impl NewSBin {
    pub fn builder<'a>() -> impl BinBuilder<'a, T = SBin> {
        DefaultBinBuilder::<NewSBin, BinBuilderCfg>::new()
    }
}

struct BinBuilderCfg;

impl BuilderCfg<SBin> for BinBuilderCfg {
    fn convert_from_sbin_to_t(sbin: SBin) -> SBin {
        sbin
    }

    fn vec_excess_capacity() -> usize {
        ArcBin::overhead_bytes()
    }
}
