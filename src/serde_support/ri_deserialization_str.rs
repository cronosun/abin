use serde::Deserializer;

use crate::serde_support::{ReIntegrationBytesVisitor, ReIntegrator, RiScope};
use crate::{
    AnyRc, AnyStr, ArcBin, Bin, RcBin, ReIntegrationStrVisitor, SBin, Str, StrReIntegrator, SyncStr,
};

/// Performs re-integration de-serialization for `Str`, see `#[serde(deserialize_with = "path")]`.
///
/// ```rust
/// use abin::Str;
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Deserialize, Serialize)]
/// pub struct ServerRequest {
///     pub request_id: u64,
///     #[serde(deserialize_with = "abin::ri_deserialize_str")]
///     pub user_name: Str,
/// }
/// ```
pub fn ri_deserialize_str<'de, D>(deserialize: D) -> Result<Str, D::Error>
where
    D: Deserializer<'de>,
{
    deserialize.deserialize_str(ReIntegrationStrVisitor::<NonSyncStrReIntegrator>::new())
}

/// Performs re-integration de-serialization for `SyncStr`, see `#[serde(deserialize_with = "path")]`.
///
/// ```rust
/// use abin::SyncStr;
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Deserialize, Serialize)]
/// pub struct ServerRequest {
///     pub request_id: u64,
///     #[serde(deserialize_with = "abin::ri_deserialize_sync_str")]
///     pub user_name: SyncStr,
/// }
/// ```
pub fn ri_deserialize_sync_str<'de, D>(deserialize: D) -> Result<SyncStr, D::Error>
where
    D: Deserializer<'de>,
{
    deserialize.deserialize_str(ReIntegrationStrVisitor::<SyncStrReIntegrator>::new())
}

/// re-integrator for `Str`.
struct NonSyncStrReIntegrator {}

impl StrReIntegrator for NonSyncStrReIntegrator {
    type TBin = Bin;

    fn re_integrate_str(str: &str) -> AnyStr<Self::TBin> {
        if let Some(bin) = RiScope::try_re_integrate(str.as_bytes()) {
            // nice, could re-integrate
            unsafe { Str::from_utf8_unchecked(bin) }
        } else {
            // bad!
            Str::from(str)
        }
    }

    fn re_integrate_string(string: String) -> AnyStr<Self::TBin> {
        // can't do much here...
        Str::from(string)
    }
}

/// re-integrator for `SyncStr`.
struct SyncStrReIntegrator {}

impl StrReIntegrator for SyncStrReIntegrator {
    type TBin = SBin;

    fn re_integrate_str(str: &str) -> AnyStr<Self::TBin> {
        if let Some(bin) = RiScope::try_re_integrate_sync(str.as_bytes()) {
            // nice, could re-integrate
            unsafe { SyncStr::from_utf8_unchecked(bin) }
        } else {
            // bad!
            SyncStr::from(str)
        }
    }

    fn re_integrate_string(string: String) -> AnyStr<Self::TBin> {
        // can't do much here...
        SyncStr::from(string)
    }
}
