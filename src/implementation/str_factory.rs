use serde::export::PhantomData;

use crate::{Bin, BooToOwned, DefaultStrBuilder, NewBin, NewSBin, SBin, SStr, Str, StrBuilder, StrFactory};

pub struct NewStr {
    _phantom: PhantomData<()>,
}

impl NewStr {
    #[inline]
    pub fn builder<'a>() -> impl StrBuilder<'a, T=Bin> {
        DefaultStrBuilder::new(NewBin::builder())
    }
}

impl StrFactory for NewStr {
    type TBinFactory = NewBin;
}

impl BooToOwned<str, Str> for NewStr {
    fn convert_to_owned(borrowed: &str) -> Str {
        Self::copy_from_str(borrowed)
    }
}

pub struct NewSStr {
    _phantom: PhantomData<()>,
}

impl NewSStr {
    #[inline]
    pub fn builder<'a>() -> impl StrBuilder<'a, T=SBin> {
        DefaultStrBuilder::new(NewSBin::builder())
    }
}

impl StrFactory for NewSStr {
    type TBinFactory = NewSBin;
}

impl BooToOwned<str, SStr> for NewSStr {
    fn convert_to_owned(borrowed: &str) -> SStr {
        Self::copy_from_str(borrowed)
    }
}
