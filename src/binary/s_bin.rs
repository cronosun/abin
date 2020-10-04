use core::fmt;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter, LowerHex, UpperHex};
use std::hash::{Hash, Hasher};
use std::ops::{Deref, RangeBounds};

use crate::{AnyBin, Bin, IntoIter, IntoSync, IntoUnSync, IntoUnSyncView, UnSyncRef, UnsafeBin};

/// A synchronized version (`Send + Sync`) of [Bin](struct.Bin.html). See
/// also [AnyBin](trait.AnyBin.html).
pub struct SBin(pub(crate) Bin);

unsafe impl Sync for SBin {}

unsafe impl Send for SBin {}

/// Returns the un-synchronized view of this binary. (so it's the same as `IntoUnSyncView`;
/// NOT `IntoUnSync`).
impl Into<Bin> for SBin {
    #[inline]
    fn into(self) -> Bin {
        self.0
    }
}

/// Returns the un-synchronized view of this binary.
impl IntoUnSyncView for SBin {
    type Target = Bin;

    #[inline]
    fn un_sync(self) -> Self::Target {
        self.0
    }
}

impl UnSyncRef for SBin {
    type Target = Bin;

    #[inline]
    fn un_sync_ref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoUnSync for SBin {
    type Target = Bin;

    #[inline]
    fn un_sync_convert(self) -> Self::Target {
        if let Some(convert_fn) = unsafe { self.0._fn_table() }.convert_into_un_sync {
            convert_fn(self.0)
        } else {
            // this means that this is already the un-synced version or there's no un-synced version.
            self.0
        }
    }
}

impl AnyBin for SBin {
    #[inline]
    fn as_slice(&self) -> &[u8] {
        self.un_sync_ref().as_slice()
    }

    #[inline]
    fn into_vec(self) -> Vec<u8> {
        self.un_sync().into_vec()
    }

    #[inline]
    fn len(&self) -> usize {
        self.un_sync_ref().len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.un_sync_ref().is_empty()
    }

    #[inline]
    fn slice<TRange>(&self, range: TRange) -> Option<Self>
    where
        TRange: RangeBounds<usize>,
    {
        self.un_sync_ref()
            .slice(range)
            .map(|bin| unsafe { bin._into_sync() })
    }

    #[inline]
    fn try_to_re_integrate(&self, slice: &[u8]) -> Option<Self> {
        unsafe {
            self.un_sync_ref()
                .try_to_re_integrate(slice)
                .map(|bin| bin._into_sync())
        }
    }
}

impl Debug for SBin {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(self.un_sync_ref(), f)
    }
}

impl Eq for SBin {}

impl PartialEq for SBin {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.un_sync_ref() == other.un_sync_ref()
    }
}

impl Ord for SBin {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.un_sync_ref().cmp(other.un_sync_ref())
    }
}

impl PartialOrd for SBin {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.un_sync_ref().partial_cmp(other.un_sync_ref())
    }
}

impl Hash for SBin {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.un_sync_ref().hash(state)
    }
}

impl Clone for SBin {
    #[inline]
    fn clone(&self) -> Self {
        unsafe { self.0.clone()._into_sync() }
    }
}

impl Borrow<[u8]> for SBin {
    #[inline]
    fn borrow(&self) -> &[u8] {
        self.un_sync_ref().as_slice()
    }
}

impl UpperHex for SBin {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        UpperHex::fmt(self.un_sync_ref(), f)
    }
}

impl LowerHex for SBin {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        LowerHex::fmt(self.un_sync_ref(), f)
    }
}

impl AsRef<[u8]> for SBin {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.un_sync_ref().as_slice()
    }
}

impl Deref for SBin {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<'a> IntoIterator for &'a SBin {
    type Item = &'a u8;
    type IntoIter = core::slice::Iter<'a, u8>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.un_sync_ref().as_slice().into_iter()
    }
}

impl IntoIterator for SBin {
    type Item = u8;
    type IntoIter = IntoIter<SBin>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self, 0)
    }
}

impl Into<Vec<u8>> for SBin {
    #[inline]
    fn into(self) -> Vec<u8> {
        self.into_vec()
    }
}

/// This is a no-op (it's already sync).
impl IntoSync for SBin {
    type Target = SBin;

    #[inline]
    fn into_sync(self) -> Self::Target {
        self
    }
}
