use core::cmp::min;
use core::fmt;
use std::fmt::Formatter;
use std::marker::PhantomData;

use serde::de;
use serde::de::Visitor;

use crate::{AnyBin, AnyStr};

pub struct ReIntegrationStrVisitor<TReIntegrator> {
    _phantom: PhantomData<TReIntegrator>,
}

impl<TReIntegrator> ReIntegrationStrVisitor<TReIntegrator> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData::default(),
        }
    }
}

pub trait StrReIntegrator {
    type TBin: AnyBin;
    fn re_integrate_str(str: &str) -> AnyStr<Self::TBin>;
    fn re_integrate_string(string: String) -> AnyStr<Self::TBin>;
}

impl<'de, TReIntegrator> Visitor<'de> for ReIntegrationStrVisitor<TReIntegrator>
    where
        TReIntegrator: StrReIntegrator,
{
    type Value = AnyStr<TReIntegrator::TBin>;

    fn expecting(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str("expecting a string")
    }

    #[inline]
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
    {
        Ok(TReIntegrator::re_integrate_str(v))
    }

    #[inline]
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: de::Error,
    {
        Ok(TReIntegrator::re_integrate_string(v))
    }
}
