use std::iter::Map;

use crate::{AnyBin, BinSegment, SegmentIterator, StrSegment};
use std::marker::PhantomData;

/// Converts a string segment iterator to a binary segment iterator.
pub struct SegmentIteratorConverter<'a, TInnerIterator, TAnyBin>
where
    TInnerIterator: SegmentIterator<StrSegment<'a, TAnyBin>>,
    TAnyBin: AnyBin,
{
    inner: TInnerIterator,
    _phantom1: PhantomData<&'a ()>,
    _phantom2: PhantomData<TAnyBin>,
}

impl<'a, TInnerIterator, TAnyBin> SegmentIteratorConverter<'a, TInnerIterator, TAnyBin>
where
    TInnerIterator: SegmentIterator<StrSegment<'a, TAnyBin>>,
    TAnyBin: AnyBin,
{
    pub fn new(string_segment_iterator: TInnerIterator) -> Self {
        Self {
            inner: string_segment_iterator,
            _phantom1: Default::default(),
            _phantom2: Default::default(),
        }
    }
}

impl<'a, TInnerIterator, TAnyBin> SegmentIterator<BinSegment<'a, TAnyBin>>
    for SegmentIteratorConverter<'a, TInnerIterator, TAnyBin>
where
    TInnerIterator: SegmentIterator<StrSegment<'a, TAnyBin>>,
    TAnyBin: AnyBin,
{
    fn exact_number_of_bytes(&self) -> Option<usize> {
        // that's the same
        self.inner.exact_number_of_bytes()
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn single(self) -> Result<BinSegment<'a, TAnyBin>, Self>
    where
        Self: Sized,
    {
        match self.inner.single() {
            Ok(single) => Ok(single.into()),
            Err(err) => Err(Self {
                inner: err,
                _phantom1: Default::default(),
                _phantom2: Default::default(),
            }),
        }
    }
}

impl<'a, TInnerIterator, TAnyBin> IntoIterator
    for SegmentIteratorConverter<'a, TInnerIterator, TAnyBin>
where
    TInnerIterator: SegmentIterator<StrSegment<'a, TAnyBin>>,
    TAnyBin: AnyBin,
{
    type Item = BinSegment<'a, TAnyBin>;

    type IntoIter = Map<
        <TInnerIterator as IntoIterator>::IntoIter,
        StrSegmentToBinSegmentConverterFn<'a, TAnyBin>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter().map(convert_fn)
    }
}

type StrSegmentToBinSegmentConverterFn<'a, TAnyBin> =
    fn(StrSegment<'a, TAnyBin>) -> BinSegment<'a, TAnyBin>;

#[inline]
fn convert_fn<TAnyBin>(str_segment: StrSegment<TAnyBin>) -> BinSegment<TAnyBin>
where
    TAnyBin: AnyBin,
{
    str_segment.into()
}
