use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Debug, Formatter, LowerHex, UpperHex};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::ops::{Bound, Deref, RangeBounds};

use crate::{
    AnyBin, BinData, FnTable, IntoIter, IntoSync, IntoUnSync, IntoUnSyncView, SBin, UnSyncRef,
    UnsafeBin,
};

/// A binary that does not implement `Send + Sync`. See `AnyBin` for documentation; see `SBin`
/// if you need `Send + Sync`. See `BinFactory` on how to create binaries.
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
        if let Some(as_slice_fn) = self.fn_table.as_slice {
            (as_slice_fn)(self)
        } else {
            &[]
        }
    }

    #[inline]
    fn into_vec(self) -> Vec<u8> {
        (self.fn_table.into_vec)(self)
    }

    #[inline]
    fn len(&self) -> usize {
        self.as_slice().len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        if let Some(is_empty_fn) = self.fn_table.is_empty {
            (is_empty_fn)(self)
        } else {
            true
        }
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

    #[inline]
    fn try_to_re_integrate(&self, slice: &[u8]) -> Option<Self> {
        if let Some(re_integrate_fn) = self.fn_table.try_re_integrate {
            (re_integrate_fn)(self, slice)
        } else {
            None
        }
    }
}

/// This does nothing, since `Bin` is already un-synchronized (view). Just returns itself.
impl IntoUnSyncView for Bin {
    type Target = Bin;

    #[inline]
    fn un_sync(self) -> Self::Target {
        self
    }
}

/// This might actually do something, since this `Bin` could just be an un-synchronized view
/// for a synchronized binary. In that case, the binary is converted.
impl IntoUnSync for Bin {
    type Target = Bin;

    #[inline]
    fn un_sync_convert(self) -> Self::Target {
        if let Some(convert_fn) = self.fn_table.convert_into_un_sync {
            convert_fn(self)
        } else {
            self
        }
    }
}

impl IntoSync for Bin {
    type Target = SBin;

    #[inline]
    fn into_sync(self) -> Self::Target {
        if let Some(convert_fn) = self.fn_table.convert_into_sync {
            unsafe { convert_fn(self)._into_sync() }
        } else {
            // this means that this is already the synced version
            unsafe { self._into_sync() }
        }
    }
}

/// This does nothing, since `Bin` is already un-synchronized (view). Just returns itself.
impl UnSyncRef for Bin {
    type Target = Bin;

    #[inline]
    fn un_sync_ref(&self) -> &Self::Target {
        &self
    }
}

impl Drop for Bin {
    #[inline]
    fn drop(&mut self) {
        if let Some(drop_fn) = self.fn_table.drop {
            (drop_fn)(self)
        }
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

impl LowerHex for Bin {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for &b in self.as_slice() {
            write!(f, "{:02x}", b)?;
        }
        Ok(())
    }
}

impl UpperHex for Bin {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for &b in self.as_slice() {
            write!(f, "{:02X}", b)?;
        }
        Ok(())
    }
}

impl AsRef<[u8]> for Bin {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl Deref for Bin {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<'a> IntoIterator for &'a Bin {
    type Item = &'a u8;
    type IntoIter = core::slice::Iter<'a, u8>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().into_iter()
    }
}

impl IntoIterator for Bin {
    type Item = u8;
    type IntoIter = IntoIter<Bin>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self, 0)
    }
}

impl Into<Vec<u8>> for Bin {
    #[inline]
    fn into(self) -> Vec<u8> {
        self.into_vec()
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
    unsafe fn _into_sync(self) -> SBin {
        SBin(self)
    }
}
