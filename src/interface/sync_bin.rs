use core::fmt;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::RangeBounds;

use crate::{AnyBin, Bin, UnsafeBin};

pub struct SyncBin(pub(crate) Bin);

unsafe impl Sync for SyncBin {}

unsafe impl Send for SyncBin {}

impl Into<Bin> for SyncBin {
    #[inline]
    fn into(self) -> Bin {
        self.0
    }
}

impl SyncBin {
    #[inline]
    pub fn un_sync(self) -> Bin {
        self.0
    }

    #[inline]
    pub fn as_bin(&self) -> &Bin {
        &self.0
    }
}

impl AnyBin for SyncBin {
    #[inline]
    fn as_slice(&self) -> &[u8] {
        self.as_bin().as_slice()
    }

    #[inline]
    fn into_vec(self) -> Vec<u8> {
        self.un_sync().into_vec()
    }

    #[inline]
    fn len(&self) -> usize {
        self.as_bin().len()
    }

    #[inline]
    fn slice<TRange>(&self, range: TRange) -> Option<Self>
        where
            TRange: RangeBounds<usize>,
    {
        self.as_bin()
            .slice(range)
            .map(|bin| unsafe { bin._into_sync() })
    }
}

impl Debug for SyncBin {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.as_bin().fmt(f)
    }
}

impl Eq for SyncBin {}

impl PartialEq for SyncBin {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_bin() == other.as_bin()
    }
}

impl Ord for SyncBin {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_bin().cmp(other.as_bin())
    }
}

impl PartialOrd for SyncBin {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_bin().partial_cmp(other.as_bin())
    }
}

impl Hash for SyncBin {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_bin().hash(state)
    }
}

impl Clone for SyncBin {
    #[inline]
    fn clone(&self) -> Self {
        unsafe { self.0.clone()._into_sync() }
    }
}

impl Borrow<[u8]> for SyncBin {
    #[inline]
    fn borrow(&self) -> &[u8] {
        self.as_bin().as_slice()
    }
}