use core::mem;
use std::iter::FromIterator;

use bytes::{Bytes, BytesMut};
use smallvec::SmallVec;

use crate::BenchStr;

#[derive(Clone)]
pub struct BytesBenchStr(Bytes);

impl BenchStr for BytesBenchStr {
    fn from_str(slice: &str) -> Self {
        Self(Bytes::copy_from_slice(slice.as_bytes()))
    }

    fn from_static(slice: &'static str) -> Self {
        Self(Bytes::from_static(slice.as_bytes()))
    }

    fn from_bin_iter(iter: impl Iterator<Item = u8>) -> Option<Self> {
        let bytes = Bytes::from_iter(iter);
        // check for utf-8 validity
        if std::str::from_utf8(bytes.as_ref()).is_ok() {
            Some(Self(bytes))
        } else {
            None
        }
    }

    fn from_multiple(iter: impl Iterator<Item = Self>) -> Self {
        let mut length_in_bytes: usize = 0;
        let mut small_vec = SmallVec::<[Self; 12]>::new();
        for item in iter {
            length_in_bytes += item.as_slice().len();
            small_vec.push(item);
        }

        let small_vec_len = small_vec.len();
        if length_in_bytes == 0 {
            Self(Bytes::new())
        } else if small_vec_len == 0 {
            mem::replace(&mut small_vec[0], Self(Bytes::new()))
        } else {
            let mut builder = BytesMut::with_capacity(length_in_bytes);
            for item in small_vec.into_iter() {
                builder.extend_from_slice(item.as_slice().as_bytes());
            }
            Self(builder.freeze())
        }
    }

    fn as_slice(&self) -> &str {
        // we know that it's valid
        let str: &str = unsafe { std::str::from_utf8_unchecked(self.0.as_ref()) };
        str
    }

    fn slice(&self, start: usize, end: usize) -> Option<Self> {
        if self.as_slice().get(start..end).is_some() {
            let bytes = self.0.slice(start..end);
            Some(Self(bytes))
        } else {
            None
        }
    }

    fn into_string(self) -> String {
        let vec = self.0.to_vec();
        // we know it's valid
        unsafe { String::from_utf8_unchecked(vec) }
    }
}
