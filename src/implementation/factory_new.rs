use std::marker::PhantomData;

use crate::{
    AnyRc, Bin, BinBuilder, BinFactory, BooToOwned, BuilderCfg, DefaultBinBuilder, IntoUnSyncView,
    RcBin, SBin,
};

/// Default implementation used to create `Bin`. See `BinFactory` for documentation.
pub struct NewBin {
    _phantom: PhantomData<()>,
}

impl NewBin {
    /// Constructs a builder that can be used to create `Bin`.
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
