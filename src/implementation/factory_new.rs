use std::marker::PhantomData;

use crate::{
    AnyRc, Bin, BinBuilder, BinFactory, BooToOwned, BuilderCfg, DefaultBinBuilder, IntoUnSyncView,
    RcBin, SBin,
};

pub struct NewBin {
    _phantom: PhantomData<()>,
}

impl NewBin {
    pub fn builder<'a>() -> impl BinBuilder<'a, T = Bin> {
        DefaultBinBuilder::<NewBin, BinBuilderCfg>::new()
    }
}

impl BooToOwned<[u8], Bin> for NewBin {
    fn convert_to_owned(borrowed: &[u8]) -> Bin {
        Self::copy_from_slice(borrowed)
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
