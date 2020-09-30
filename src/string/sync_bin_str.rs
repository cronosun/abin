use crate::{AnyRc, AnyStr, ArcBin, StaticBin, Str, SyncBin, UnSync};

/// A string backed by `SyncBin` (sync + send).
pub type SyncStr = AnyStr<SyncBin>;

impl SyncStr {
    #[inline]
    pub fn from_static(string: &'static str) -> AnyStr<SyncBin> {
        let static_bin = StaticBin::from(string.as_bytes());
        unsafe { Self::from_utf8_unchecked(static_bin) }
    }
}

impl From<String> for SyncStr {
    fn from(string: String) -> Self {
        let bytes = string.into_bytes();
        let bin = ArcBin::from_vec(bytes);
        unsafe { SyncStr::from_utf8_unchecked(bin) }
    }
}

impl<'a> From<&'a str> for SyncStr {
    fn from(string: &'a str) -> Self {
        let bin = ArcBin::copy_from_slice(string.as_bytes());
        unsafe { SyncStr::from_utf8_unchecked(bin) }
    }
}

impl UnSync for SyncStr {
    type Target = Str;

    fn un_sync(self) -> Self::Target {
        let binary: SyncBin = self.into_bin();
        let binary = binary.un_sync();
        unsafe { Str::from_utf8_unchecked(binary) }
    }
}
