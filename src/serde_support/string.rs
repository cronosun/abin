use core::fmt;
use std::fmt::Formatter;
use std::marker::PhantomData;

use serde::de::Visitor;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use crate::{AnyBin, AnyRc, AnyStr, ArcBin, RcBin, SStr, Str};

impl Serialize for Str {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for Str {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(RcStrVisitor::<RcBin>::new())
    }
}

impl Serialize for SStr {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for SStr {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(RcStrVisitor::<ArcBin>::new())
    }
}

struct RcStrVisitor<T> {
    _phantom: PhantomData<T>,
}

impl<T> RcStrVisitor<T> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData::default(),
        }
    }
}

impl<'de, T> Visitor<'de> for RcStrVisitor<T>
where
    T: AnyRc,
    T::T: AnyBin,
{
    type Value = AnyStr<T::T>;

    fn expecting(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str("expecting a string")
    }

    #[inline]
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let bin = T::copy_from_slice(v.as_bytes());
        // we know it's safe (we just got the valid str)
        unsafe { Ok(AnyStr::from_utf8_unchecked(bin)) }
    }

    #[inline]
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let bin = T::from_vec(v.into_bytes());
        // we know it's safe (we just got the valid str)
        unsafe { Ok(AnyStr::from_utf8_unchecked(bin)) }
    }
}
