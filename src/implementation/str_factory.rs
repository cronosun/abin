use serde::export::PhantomData;

use crate::{Bin, DefaultStrBuilder, NewBin, NewSBin, SBin, StrBuilder, StrFactory};

pub struct NewStr {
    _phantom: PhantomData<()>,
}

impl NewStr {
    #[inline]
    pub fn builder<'a>() -> impl StrBuilder<'a, T = Bin> {
        DefaultStrBuilder::new(NewBin::builder())
    }
}

impl StrFactory for NewStr {
    type TBinFactory = NewBin;
}

pub struct NewSStr {
    _phantom: PhantomData<()>,
}

impl NewSStr {
    #[inline]
    pub fn builder<'a>() -> impl StrBuilder<'a, T = SBin> {
        DefaultStrBuilder::new(NewSBin::builder())
    }
}

impl StrFactory for NewSStr {
    type TBinFactory = NewSBin;
}
