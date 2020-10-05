use crate::{Bin, Boo, SBin, SStr, Str};

pub type BooBin<'a> = Boo<'a, [u8], Bin>;
pub type BooSBin<'a> = Boo<'a, [u8], SBin>;
pub type BooStr<'a> = Boo<'a, str, Str>;
pub type BooSStr<'a> = Boo<'a, str, SStr>;
