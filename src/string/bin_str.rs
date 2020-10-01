use crate::{AnyRc, AnyStr, Bin, IntoSync, IntoUnSyncView, RcBin, SyncStr};

/// A string backed by [Bin](struct.Bin.html) (not `Sync + Send`).
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
