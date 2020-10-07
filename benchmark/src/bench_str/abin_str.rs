use abin::{NewStr, Str, StrBuilder, StrFactory, StrSegment};

use crate::BenchStr;

#[derive(Clone)]
pub struct StrBenchStr(Str);

impl BenchStr for StrBenchStr {
    fn from_str(slice: &str) -> Self {
        Self(NewStr::copy_from_str(slice))
    }

    fn from_static(slice: &'static str) -> Self {
        Self(NewStr::from_static(slice))
    }

    fn from_bin_iter(iter: impl Iterator<Item = u8>) -> Option<Self> {
        if let Ok(str) = NewStr::from_utf8_iter(iter) {
            Some(Self(str))
        } else {
            None
        }
    }

    fn from_multiple(iter: impl Iterator<Item = Self>) -> Self {
        let mut builder = NewStr::builder();
        for item in iter {
            builder.push(StrSegment::Str(item.0));
        }
        let str = builder.build();
        Self(str)
    }

    fn as_slice(&self) -> &str {
        self.0.as_str()
    }

    fn slice(&self, start: usize, end: usize) -> Option<Self> {
        self.0.slice(start..end).map(Self)
    }

    fn into_string(self) -> String {
        self.0.into_string()
    }
}
