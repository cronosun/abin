use crate::{Bin, Boo, SBin, SStr, Str};

/// Borrowed-or-owned `Bin`.
pub type BooBin<'a> = Boo<'a, [u8], Bin>;
/// Borrowed-or-owned `SBin`.
pub type BooSBin<'a> = Boo<'a, [u8], SBin>;
/// Borrowed-or-owned `Str`.
pub type BooStr<'a> = Boo<'a, str, Str>;
/// Borrowed-or-owned `SStr`.
pub type BooSStr<'a> = Boo<'a, str, SStr>;
