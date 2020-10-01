use core::cmp::min;
use core::fmt;

use serde::de::Visitor;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use crate::{AnyBin, AnyRc, ArcBin, Bin, RcBin, SyncBin};
use std::fmt::Formatter;
use std::marker::PhantomData;

impl Serialize for Bin {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(self.as_slice())
    }
}

impl<'de> Deserialize<'de> for Bin {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        // we use the ref count binary here; alternative would be the VecBin: ... but I think the
        // advantages of a RcBin outweighs the disadvantage of the reference-counter.
        deserializer.deserialize_bytes(RcBytesVisitor::<RcBin>::new())
    }
}

impl Serialize for SyncBin {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(self.as_slice())
    }
}

impl<'de> Deserialize<'de> for SyncBin {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        // we use the ref count binary here; alternative would be the VecBin: ... but I think the
        // advantages of a ArcBin outweighs the disadvantage of the reference-counter.
        deserializer.deserialize_bytes(RcBytesVisitor::<ArcBin>::new())
    }
}

struct RcBytesVisitor<T> {
    _phantom: PhantomData<T>,
}

impl<T> RcBytesVisitor<T> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData::default(),
        }
    }
}

static SAFE_MAX_LEN: usize = 256 * 1024;
static GUESSED_LEN: usize = 256;

impl<'de, T> Visitor<'de> for RcBytesVisitor<T>
where
    T: AnyRc,
{
    type Value = T::T;

    fn expecting(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str("expecting a byte array")
    }

    #[inline]
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(T::copy_from_slice(v.as_bytes()))
    }

    #[inline]
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(T::from_vec(v.into_bytes()))
    }

    #[inline]
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(T::copy_from_slice(v))
    }

    #[inline]
    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(T::from_vec(v))
    }

    #[inline]
    fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
    where
        V: de::SeqAccess<'de>,
    {
        let capacity = if let Some(known_len) = seq.size_hint() {
            // note: We limit it to 'SAFE_MAX_LEN' in case there's a problem with the input data.
            min(known_len, SAFE_MAX_LEN)
        } else {
            // the length is now known; set some minimum
            GUESSED_LEN
        };
        // we extend the capacity, but only if its > 0. why? If it's 0, it's an empty sequence,
        // and an empty sequence produces an empty Vec<u8>; and the empty Vec<u8> does not
        // allocate and can be stack-allocated.
        let capacity = if capacity > 0 {
            capacity + T::overhead_bytes()
        } else {
            capacity
        };
        let mut values: Vec<u8> = Vec::with_capacity(capacity);

        while let Some(value) = seq.next_element()? {
            values.push(value);
        }

        Ok(T::from_vec(values))
    }
}
