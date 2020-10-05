use core::mem;

use crate::common::segment_iterator::SegmentIterator;
use crate::Segment;

/// It's an implementation of `SegmentIterator` that does not heap-allocate. Alternatives are
/// `BinBuilder` and `StrBuilder`; `SegmentsSlice` is less flexible but cheaper
/// (smaller stack; faster).
///
/// ```rust
/// use abin::{BinSegment, SegmentsSlice, Bin, NewBin, BinFactory, AnyBin};
///
/// let segments = &mut [BinSegment::Static("Hello, ".as_bytes()),
///     BinSegment::Static("World!".as_bytes())];
/// let iterator = SegmentsSlice::new(segments);
/// let bin : Bin = NewBin::from_segments(iterator);
/// assert_eq!("Hello, World!".as_bytes(), bin.as_slice());
/// ```
pub struct SegmentsSlice<'a, TSegment> {
    slice: &'a mut [TSegment],
    number_of_bytes: usize,
    pos: usize,
    /// this is set to the index of the single item (if there's just one single item).
    single_index: Option<usize>,
}

impl<'a, TSegment> SegmentsSlice<'a, TSegment>
where
    TSegment: Segment,
{
    /// Important: The given slice might be modified; do not use this slice.
    #[inline]
    pub fn new(slice: &'a mut [TSegment]) -> Self {
        // analyze content.
        let mut number_of_bytes = 0;
        let mut no_item_yet = true;
        let mut single_index = None;
        for (index, item) in slice.iter().enumerate() {
            let item_len = item.number_of_bytes();
            // ignore all empty items
            if item_len > 0 {
                number_of_bytes += item_len;
                if no_item_yet {
                    single_index = Some(index);
                    no_item_yet = false;
                } else {
                    // more than one
                    single_index = None;
                }
            }
        }

        Self {
            slice,
            number_of_bytes,
            pos: 0,
            single_index,
        }
    }

    #[inline]
    fn remaining(&self) -> usize {
        let len = self.slice.len();
        if self.pos < len {
            len - self.pos
        } else {
            0
        }
    }

    fn take(&mut self, index: usize) -> Option<TSegment> {
        if index < self.slice.len() {
            let empty = TSegment::empty();
            Some(mem::replace(&mut self.slice[index], empty))
        } else {
            None
        }
    }
}

impl<'a, TSegment> SegmentIterator<TSegment> for SegmentsSlice<'a, TSegment>
where
    TSegment: Segment,
{
    fn exact_number_of_bytes(&self) -> Option<usize> {
        Some(self.number_of_bytes)
    }

    fn is_empty(&self) -> bool {
        self.number_of_bytes == 0
    }

    fn single(mut self) -> Result<TSegment, Self>
    where
        Self: Sized,
    {
        if let Some(single_index) = self.single_index {
            let taken = SegmentsSlice::take(&mut self, single_index);
            Ok(taken.expect("Implementation error (single_index is invalid)"))
        } else {
            Err(self)
        }
    }
}

impl<'a, TSegment: Segment> Iterator for SegmentsSlice<'a, TSegment> {
    type Item = TSegment;

    fn next(&mut self) -> Option<Self::Item> {
        let item = SegmentsSlice::take(self, self.pos);
        self.pos += 1;
        item
    }
}

impl<'a, TSegment: Segment> ExactSizeIterator for SegmentsSlice<'a, TSegment> {
    #[inline]
    fn len(&self) -> usize {
        self.remaining()
    }
}
