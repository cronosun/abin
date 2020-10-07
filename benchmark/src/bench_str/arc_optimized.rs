use core::mem;
use std::ops::Deref;
use std::sync::Arc;

use smallvec::SmallVec;

use crate::BenchStr;

/// a hand-optimized version using things from std-lib; should have about the same performance
/// as `SStr` from `abin` (maybe even a bit better, since has less functionality; less overhead).
#[derive(Clone)]
pub enum StdLibOptimized {
    Empty,
    Static(&'static str, usize, usize),
    ArcStr(Arc<str>, usize, usize),
    ArcString(Arc<String>, usize, usize),
}

impl StdLibOptimized {
    fn from_given_string(string: String) -> Self {
        let len = string.len();
        if len == 0 {
            Self::Empty
        } else if len > 1024 {
            Self::ArcString(Arc::new(string), 0, len)
        } else {
            Self::ArcStr(Arc::from(string.deref()), 0, len)
        }
    }
}

impl BenchStr for StdLibOptimized {
    fn from_str(slice: &str) -> Self {
        let len = slice.len();
        if len == 0 {
            Self::Empty
        } else {
            Self::ArcStr(Arc::from(slice), 0, len)
        }
    }

    fn from_static(slice: &'static str) -> Self {
        Self::Static(slice, 0, slice.len())
    }

    fn from_bin_iter(iter: impl Iterator<Item = u8>) -> Option<Self>
    where
        Self: Sized,
    {
        let vec: Vec<u8> = iter.collect();
        let string = String::from_utf8(vec);
        if let Ok(string) = string {
            Some(Self::from_given_string(string))
        } else {
            None
        }
    }

    fn from_multiple(iter: impl Iterator<Item = Self>) -> Self
    where
        Self: Sized,
    {
        let mut length_in_bytes: usize = 0;
        let mut small_vec = SmallVec::<[Self; 12]>::new();
        for item in iter {
            length_in_bytes += item.as_slice().len();
            small_vec.push(item);
        }

        let small_vec_len = small_vec.len();
        if length_in_bytes == 0 {
            Self::Empty
        } else if small_vec_len == 0 {
            mem::replace(&mut small_vec[0], Self::Empty)
        } else {
            let mut string = String::with_capacity(length_in_bytes);
            for item in small_vec.into_iter() {
                string.push_str(item.as_slice());
            }
            Self::from_given_string(string)
        }
    }

    fn as_slice(&self) -> &str {
        match self {
            Self::Static(value, start, end) => &value[*start..*end],
            Self::ArcStr(value, start, end) => &value.deref()[*start..*end],
            Self::ArcString(value, start, end) => &value.deref()[*start..*end],
            Self::Empty => "",
        }
    }

    fn slice(&self, start: usize, end: usize) -> Option<Self>
    where
        Self: Sized,
    {
        match self {
            Self::Static(value, this_start, this_end) => {
                let new_start = this_start + start;
                let new_end = this_end + end;
                value
                    .get(new_start..new_end)
                    .map(|_| Self::Static(value, new_start, new_end))
            }
            Self::ArcStr(value, this_start, this_end) => {
                let new_start = this_start + start;
                let new_end = this_end + end;

                if value.get(new_start..new_end).is_some() {
                    Some(Self::ArcStr(value.clone(), new_start, new_end))
                } else {
                    None
                }
            }
            Self::ArcString(value, this_start, this_end) => {
                let new_start = this_start + start;
                let new_end = this_end + end;

                if value.get(new_start..new_end).is_some() {
                    Some(Self::ArcString(value.clone(), new_start, new_end))
                } else {
                    None
                }
            }
            Self::Empty => {
                if start == 0 && end == 0 {
                    Some(Self::Empty)
                } else {
                    None
                }
            }
        }
    }

    fn into_string(self) -> String {
        match self {
            Self::Static(_, _, _) => self.as_slice().to_owned(),
            Self::ArcStr(_, _, _) => {
                let slice: &str = self.as_slice();
                slice.to_owned()
            }
            Self::ArcString(value, start, end) => {
                let len = value.len();
                let entire_range = start == 0 && end == len;
                if entire_range {
                    match Arc::try_unwrap(value) {
                        Ok(string) => string,
                        Err(original) => {
                            let slice: &str = original.deref();
                            slice.to_owned()
                        }
                    }
                } else {
                    let slice: &str = value.deref();
                    let slice = &slice[start..end];
                    slice.to_owned()
                }
            }
            Self::Empty => "".to_owned(),
        }
    }
}
