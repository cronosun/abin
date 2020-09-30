use crate::{AnyRc, AnyStr, Bin, RcBin, UnSync};

/// A string backed by `Bin` (not sync + send).
pub type Str = AnyStr<Bin>;

impl From<String> for Str {
    fn from(string: String) -> Self {
        let bytes = string.into_bytes();
        let bin = RcBin::from_vec(bytes);
        unsafe { Str::from_utf8_unchecked(bin) }
    }
}

impl<'a> From<&'a str> for Str {
    fn from(string: &'a str) -> Self {
        let bin = RcBin::copy_from_slice(string.as_bytes());
        unsafe { Str::from_utf8_unchecked(bin) }
    }
}

impl UnSync for Str {
    type Target = Str;

    fn un_sync(self) -> Self::Target {
        self
    }
}
