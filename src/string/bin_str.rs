use crate::{AnyRc, AnyStr, Bin, IntoSync, IntoUnSyncView, RcBin, StaticBin, SyncStr};

/// A string backed by [Bin](struct.Bin.html) (not `Sync + Send`).
pub type Str = AnyStr<Bin>;

impl Str {
    /// Static string backed by [StaticBin](struct.StaticBin.html).
    #[inline]
    pub fn from_static(string: &'static str) -> Self {
        let static_bin = StaticBin::from(string.as_bytes()).un_sync();
        unsafe { Self::from_utf8_unchecked(static_bin) }
    }
}

impl From<String> for Str {
    fn from(string: String) -> Self {
        let bytes = string.into_bytes();
        let bin = RcBin::from_vec(bytes);
        unsafe { Self::from_utf8_unchecked(bin) }
    }
}

impl<'a> From<&'a str> for Str {
    fn from(string: &'a str) -> Self {
        let bin = RcBin::copy_from_slice(string.as_bytes());
        unsafe { Self::from_utf8_unchecked(bin) }
    }
}

impl IntoUnSyncView for Str {
    type Target = Str;

    fn un_sync(self) -> Self::Target {
        self
    }
}

impl IntoSync for Str {
    type Target = SyncStr;

    fn into_sync(self) -> Self::Target {
        let bin = self.into_bin();
        let sync_bin = bin.into_sync();
        unsafe { Self::Target::from_utf8_unchecked(sync_bin) }
    }
}
