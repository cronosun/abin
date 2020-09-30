use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::ops::{Bound, RangeBounds};

use crate::{AnyBin, BinData, FnTable, SyncBin, UnSync, UnSyncRef, UnsafeBin};
use std::borrow::Borrow;

#[repr(C)]
pub struct Bin {
    data: BinData,
    fn_table: &'static FnTable,
    // marker to make sure this is not send + sync
    _not_sync: PhantomData<*const u8>,
}

impl AnyBin for Bin {
    #[inline]
    fn as_slice(&self) -> &[u8] {
        (self.fn_table.as_slice)(self)
    }

    #[inline]
    fn into_vec(self) -> Vec<u8> {
        (self.fn_table.into_vec)(self)
    }

    #[inline]
    fn len(&self) -> usize {
        (self.fn_table.as_slice)(self).len()
    }

    #[inline]
    fn slice<TRange>(&self, range: TRange) -> Option<Self>
    where
        TRange: RangeBounds<usize>,
    {
        let start = match range.start_bound() {
            Bound::Included(start) => *start,
            Bound::Excluded(start) => *start + 1,
            Bound::Unbounded => 0,
        };
        let end_excluded = match range.end_bound() {
            Bound::Included(end) => *end - 1,
            Bound::Excluded(end) => *end,
            Bound::Unbounded => self.len(),
        };
        (self.fn_table.slice)(self, start, end_excluded)
    }
}

/// This does nothing, since `Bin` is already un-synchronized. Just returns itself.
impl UnSync for Bin {
    type Target = Bin;

    #[inline]
    fn un_sync(self) -> Self::Target {
        self
    }
}

/// This does nothing, since `Bin` is already un-synchronized. Just returns itself.
impl UnSyncRef for Bin {
    type Target = Bin;

    fn un_sync_ref(&self) -> &Self::Target {
        &self
    }
}

impl Drop for Bin {
    #[inline]
    fn drop(&mut self) {
        (self.fn_table.drop)(self)
    }
}

impl Clone for Bin {
    #[inline]
    fn clone(&self) -> Self {
        (self.fn_table.clone)(self)
    }
}

impl Debug for Bin {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.as_slice().fmt(f)
    }
}

impl Eq for Bin {}

impl PartialEq for Bin {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }

    #[inline]
    fn ne(&self, other: &Self) -> bool {
        self.as_slice().ne(other.as_slice())
    }
}

impl Ord for Bin {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}

impl PartialOrd for Bin {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_slice().partial_cmp(other.as_slice())
    }
}

impl Hash for Bin {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state)
    }
}

impl Borrow<[u8]> for Bin {
    #[inline]
    fn borrow(&self) -> &[u8] {
        self.as_slice()
    }
}

impl Bin {
    /// This is required since we can't use `unsafe` in const fn but we need const new
    /// for the empty bin.
    pub(crate) const fn _const_new(data: BinData, fn_table: &'static FnTable) -> Self {
        Self {
            data,
            fn_table,
            _not_sync: PhantomData,
        }
    }
}

unsafe impl UnsafeBin for Bin {
    #[inline]
    unsafe fn _new(data: BinData, fn_table: &'static FnTable) -> Self {
        Self {
            data,
            fn_table,
            _not_sync: PhantomData,
        }
    }

    #[inline]
    unsafe fn _data(&self) -> &BinData {
        &self.data
    }

    #[inline]
    unsafe fn _data_mut(&mut self) -> &mut BinData {
        &mut self.data
    }

    #[inline]
    unsafe fn _fn_table(&self) -> &'static FnTable {
        self.fn_table
    }

    #[inline]
    unsafe fn _into_sync(self) -> SyncBin {
        SyncBin(self)
    }
}
