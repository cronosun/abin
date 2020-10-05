use std::marker::PhantomData;

use crate::{
    AnyRc, ArcBin, BinBuilder, BinFactory, BooToOwned, BuilderCfg, DefaultBinBuilder, SBin,
};

/// Default implementation used to create `SBin`. See `BinFactory` for documentation.
///
/// ```rust
/// use abin::{NewSBin, SBin, BinFactory, AnyBin};
/// let bin : SBin = NewSBin::from_static("Hello, I'm a binary!".as_bytes());
/// assert_eq!("Hello, I'm a binary!".as_bytes(), bin.as_slice());
/// ```
pub struct NewSBin {
    _phantom: PhantomData<()>,
}

impl NewSBin {
    /// Constructs a builder that can be used to create `SBin`.
    pub fn builder<'a>() -> impl BinBuilder<'a, T = SBin> {
        DefaultBinBuilder::<NewSBin, BinBuilderCfg>::new()
    }
}

impl BooToOwned<[u8], SBin> for NewSBin {
    fn convert_to_owned(borrowed: &[u8]) -> SBin {
        Self::copy_from_slice(borrowed)
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
