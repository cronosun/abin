use serde::Deserializer;

use crate::serde_support::{ReIntegrationBytesVisitor, ReIntegrator, RiScope};
use crate::{AnyRc, ArcBin, Bin, RcBin, SBin};

/// Performs re-integration de-serialization for `Bin`, see `#[serde(deserialize_with = "path")]`.
///
/// ```rust
/// use serde::{Deserialize, Serialize};
/// use abin::Bin;
///
/// #[derive(Deserialize, Serialize)]
/// struct SomeData {
///   some_number : usize,
///   #[serde(deserialize_with = "abin::ri_deserialize_bin")]
///   binary : Bin
/// }
/// ```
pub fn ri_deserialize_bin<'de, D>(deserialize: D) -> Result<Bin, D::Error>
    where
        D: Deserializer<'de>,
{
    deserialize.deserialize_bytes(ReIntegrationBytesVisitor::<BinReIntegrator>::new())
}

/// Performs re-integration de-serialization for `SyncBin`, see `#[serde(deserialize_with = "path")]`.
///
/// ```rust
/// use serde::{Deserialize, Serialize};
/// use abin::SBin;
///
/// #[derive(Deserialize, Serialize)]
/// struct SomeData {
///   some_number : usize,
///   #[serde(deserialize_with = "abin::ri_deserialize_sync_bin")]
///   binary : SBin
/// }
/// ```
pub fn ri_deserialize_sync_bin<'de, D>(deserialize: D) -> Result<SBin, D::Error>
    where
        D: Deserializer<'de>,
{
    deserialize.deserialize_bytes(ReIntegrationBytesVisitor::<SyncBinReIntegrator>::new())
}

/// re-integrator for `Bin`.
struct BinReIntegrator {}

impl ReIntegrator for BinReIntegrator {
    type TBin = Bin;

    #[inline]
    fn re_integrate(slice: &[u8]) -> Self::TBin {
        if let Some(bin) = RiScope::try_re_integrate(slice) {
            // nice, could re-integrate
            bin
        } else {
            // bad!
            RcBin::copy_from_slice(slice)
        }
    }

    #[inline]
    fn vec(vec: Vec<u8>) -> Self::TBin {
        // can't do much here...
        RcBin::from_vec(vec)
    }

    #[inline]
    fn overhead_bytes_for_vec() -> usize {
        RcBin::overhead_bytes()
    }
}

/// re-integrator for `SyncBin`.
struct SyncBinReIntegrator {}

impl ReIntegrator for SyncBinReIntegrator {
    type TBin = SBin;

    #[inline]
    fn re_integrate(slice: &[u8]) -> Self::TBin {
        if let Some(bin) = RiScope::try_re_integrate_sync(slice) {
            // nice, could re-integrate
            bin
        } else {
            // bad!
            ArcBin::copy_from_slice(slice)
        }
    }

    #[inline]
    fn vec(vec: Vec<u8>) -> Self::TBin {
        // can't do much here...
        ArcBin::from_vec(vec)
    }

    #[inline]
    fn overhead_bytes_for_vec() -> usize {
        ArcBin::overhead_bytes()
    }
}
