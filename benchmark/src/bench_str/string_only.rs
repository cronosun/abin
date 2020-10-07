use crate::BenchStr;

/// a non-optimized naive implementation just using `String` from the std-lib.
#[derive(Clone)]
pub struct StdLibStringOnly(String);

impl BenchStr for StdLibStringOnly {
    fn from_str(slice: &str) -> Self {
        Self(String::from(slice))
    }

    fn from_static(slice: &'static str) -> Self {
        Self(String::from(slice))
    }

    fn from_bin_iter(iter: impl Iterator<Item = u8>) -> Option<Self>
    where
        Self: Sized,
    {
        let vec: Vec<u8> = iter.collect();
        if let Ok(string) = String::from_utf8(vec) {
            Some(Self(string))
        } else {
            None
        }
    }

    fn from_multiple(iter: impl Iterator<Item = Self>) -> Self
    where
        Self: Sized,
    {
        let string: String = iter.map(|item| item.0).collect();
        Self(string)
    }

    fn as_slice(&self) -> &str {
        self.0.as_str()
    }

    fn slice(&self, start: usize, end: usize) -> Option<Self>
    where
        Self: Sized,
    {
        if let Some(slice) = self.0.get(start..end) {
            Some(Self(slice.to_owned()))
        } else {
            None
        }
    }

    fn into_string(self) -> String {
        self.0
    }
}
