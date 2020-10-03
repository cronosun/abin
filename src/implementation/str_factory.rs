use serde::export::PhantomData;

use crate::{New, SNew, StrFactory};

pub struct NewStr {
    _phantom: PhantomData<()>,
}

impl StrFactory for NewStr {
    type TBinFactory = New;
}

pub struct NewSStr {
    _phantom: PhantomData<()>,
}

impl StrFactory for NewSStr {
    type TBinFactory = SNew;
}
