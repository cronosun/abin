use core::cmp::min;
use core::fmt;
use std::fmt::Formatter;
use std::marker::PhantomData;

use serde::de;
use serde::de::Visitor;

use crate::AnyBin;

static SAFE_MAX_LEN: usize = 256 * 1024;
static GUESSED_LEN: usize = 256;

pub struct ReIntegrationBytesVisitor<TReIntegrator> {
    _phantom: PhantomData<TReIntegrator>,
}

impl<TReIntegrator> ReIntegrationBytesVisitor<TReIntegrator> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData::default(),
        }
    }
}

pub trait ReIntegrator {
    type TBin: AnyBin;
    fn re_integrate(slice: &[u8]) -> Self::TBin;
    fn vec(vec: Vec<u8>) -> Self::TBin;
    fn overhead_bytes_for_vec() -> usize;
}

impl<'de, TReIntegrator> Visitor<'de> for ReIntegrationBytesVisitor<TReIntegrator>
where
    TReIntegrator: ReIntegrator,
{
    type Value = TReIntegrator::TBin;

    fn expecting(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str("expecting a byte array")
    }

    #[inline]
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(TReIntegrator::re_integrate(v.as_bytes()))
    }

    #[inline]
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(TReIntegrator::vec(v.into_bytes()))
    }

    #[inline]
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(TReIntegrator::re_integrate(v))
    }

    #[inline]
    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(TReIntegrator::vec(v))
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
            capacity + TReIntegrator::overhead_bytes_for_vec()
        } else {
            capacity
        };
        let mut values: Vec<u8> = Vec::with_capacity(capacity);

        while let Some(value) = seq.next_element()? {
            values.push(value);
        }

        Ok(TReIntegrator::vec(values))
    }
}
