use serde::export::PhantomData;

use crate::{NewBin, NewSBin, StrFactory};

pub struct NewStr {
    _phantom: PhantomData<()>,
}

impl StrFactory for NewStr {
    type TBinFactory = NewBin;
}

pub struct NewSStr {
    _phantom: PhantomData<()>,
}

impl StrFactory for NewSStr {
    type TBinFactory = NewSBin;
}
