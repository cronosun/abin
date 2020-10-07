use crate::BenchStr;
use std::ops::Deref;
use std::sync::Arc;

/// a not very optimized version - just using `Arc<str>`.
#[derive(Clone)]
pub struct StdLibArcStrOnly(Arc<str>);

impl BenchStr for StdLibArcStrOnly {
    fn from_str(slice: &str) -> Self {
        Self(Arc::from(slice))
    }

    fn from_static(slice: &'static str) -> Self {
        Self(Arc::from(slice))
    }

    fn from_bin_iter(iter: impl Iterator<Item = u8>) -> Option<Self>
    where
        Self: Sized,
    {
        let vec: Vec<u8> = iter.collect();
        if let Ok(string) = String::from_utf8(vec) {
            Some(Self(string.into()))
        } else {
            None
        }
    }

    fn from_multiple(iter: impl Iterator<Item = Self>) -> Self
    where
        Self: Sized,
    {
        let string: String = iter.map(|item| item.into_string()).collect();
        Self(string.into())
    }

    fn as_slice(&self) -> &str {
        self.0.deref()
    }

    fn slice(&self, start: usize, end: usize) -> Option<Self>
    where
        Self: Sized,
    {
        if let Some(slice) = self.0.get(start..end) {
            Some(Self(slice.into()))
        } else {
            None
        }
    }

    fn into_string(self) -> String {
        self.0.deref().to_owned()
    }
}
