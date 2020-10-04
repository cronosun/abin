use crate::{AnyStr, BinBuilder, BinSegment, StrBuilder, StrSegment};

pub struct DefaultStrBuilder<TBinBuilder> {
    bin_builder: TBinBuilder,
}

impl<TBinBuilder> DefaultStrBuilder<TBinBuilder> {
    pub fn new(bin_builder: TBinBuilder) -> Self {
        Self { bin_builder }
    }
}

impl<'a, TBinBuilder> StrBuilder<'a> for DefaultStrBuilder<TBinBuilder>
where
    TBinBuilder: BinBuilder<'a>,
{
    type T = TBinBuilder::T;

    #[inline]
    fn push(&mut self, segment: impl Into<StrSegment<'a, Self::T>>) {
        let str_segment = segment.into();
        let bin_segment: BinSegment<'a, Self::T> = str_segment.into();
        self.bin_builder.push(bin_segment);
    }

    #[inline]
    fn build(&mut self) -> AnyStr<Self::T> {
        let bin = self.bin_builder.build();
        // we know that bin only contains valid UTF-8, since all segments are valid UTF-8 and
        // concatenating valid UTF-8 segments should always produce valid UTF-8.
        unsafe { AnyStr::from_utf8_unchecked(bin) }
    }
}
