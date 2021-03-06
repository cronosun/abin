use serde::Deserializer;

use crate::serde_support::RiScope;
use crate::{
    AnyStr, Bin, NewSStr, NewStr, ReIntegrationStrVisitor, SBin, SStr, Str, StrFactory,
    StrReIntegrator,
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

/// Performs re-integration de-serialization for `SStr`, see `#[serde(deserialize_with = "path")]`.
///
/// ```rust
/// use abin::SyncStr;
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Deserialize, Serialize)]
/// pub struct ServerRequest {
///     pub request_id: u64,
///     #[serde(deserialize_with = "abin::ri_deserialize_sstr")]
///     pub user_name: SyncStr,
/// }
/// ```
pub fn ri_deserialize_sstr<'de, D>(deserialize: D) -> Result<SStr, D::Error>
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
            NewStr::copy_from_str(str)
        }
    }

    fn re_integrate_string(string: String) -> AnyStr<Self::TBin> {
        // can't do much here...
        NewStr::from_given_string(string)
    }
}

/// re-integrator for `SyncStr`.
struct SyncStrReIntegrator {}

impl StrReIntegrator for SyncStrReIntegrator {
    type TBin = SBin;

    fn re_integrate_str(str: &str) -> AnyStr<Self::TBin> {
        if let Some(bin) = RiScope::try_re_integrate_sync(str.as_bytes()) {
            // nice, could re-integrate
            unsafe { SStr::from_utf8_unchecked(bin) }
        } else {
            // bad!
            NewSStr::copy_from_str(str)
        }
    }

    fn re_integrate_string(string: String) -> AnyStr<Self::TBin> {
        // can't do much here...
        NewSStr::from_given_string(string)
    }
}
