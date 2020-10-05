use std::borrow::Borrow;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::Boo;

impl<'a, TBorrowed, TOwned> Serialize for Boo<'a, TBorrowed, TOwned>
where
    TBorrowed: ?Sized,
    TBorrowed: Serialize,
    TOwned: Borrow<TBorrowed>,
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let borrowed: &TBorrowed = self.borrow();
        borrowed.serialize(serializer)
    }
}

impl<'de: 'a, 'a, TBorrowed, TOwned> Deserialize<'de> for Boo<'a, TBorrowed, TOwned>
where
    TBorrowed: ?Sized,
    &'a TBorrowed: Deserialize<'de>,
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        // we always de-serialize borrowed (that's what you usually want to do).
        let borrowed = <(&'a TBorrowed)>::deserialize(deserializer)?;
        Ok(Boo::Borrowed(borrowed))
    }
}
