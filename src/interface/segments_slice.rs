use core::mem;

use crate::{AnyBin, BinSegment};
use crate::interface::segments_iterator::SegmentsIterator;

pub struct SegmentsSlice<'a, TAnyBin: AnyBin> {
    slice: &'a mut [BinSegment<'a, TAnyBin>],
    number_of_bytes: usize,
    pos: usize,
    /// this is set to the index of the single item (if there's just one single item).
    single_index: Option<usize>,
}

impl<'a, TAnyBin: AnyBin> SegmentsSlice<'a, TAnyBin> {
    /// Important: The given slice might be modified; do not use this slice.
    #[inline]
    pub fn new(slice: &'a mut [BinSegment<'a, TAnyBin>]) -> Self {
        // analyze content.
        let mut number_of_bytes = 0;
        let mut no_item_yet = true;
        let mut single_index = None;
        for (index, item) in slice.iter().enumerate() {
            let item_len = item.as_slice().len();
            // ignore all empty items
            if item_len > 0 {
                number_of_bytes += item_len;
                if no_item_yet {
                    single_index = Some(index);
                }
                no_item_yet = false;
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

    fn take(&mut self, index: usize) -> Option<BinSegment<'a, TAnyBin>> {
        if index < self.slice.len() {
            Some(mem::replace(&mut self.slice[index], BinSegment::Empty))
        } else {
            None
        }
    }
}

impl<'a, TAnyBin: AnyBin> SegmentsIterator<'a, TAnyBin> for SegmentsSlice<'a, TAnyBin> {
    type TAnyBin = TAnyBin;

    fn exact_number_of_bytes(&self) -> Option<usize> {
        Some(self.number_of_bytes)
    }

    fn is_empty(&self) -> bool {
        self.number_of_bytes == 0
    }

    fn single(mut self) -> Result<BinSegment<'a, TAnyBin>, Self> where
        Self: Sized {
        if let Some(single_index) = self.single_index {
            let taken = SegmentsSlice::take(&mut self, single_index);
            Ok(taken.expect("Implementation error (single_index is invalid)"))
        } else {
            Err(self)
        }
    }
}

impl<'a, TAnyBin: AnyBin> Iterator for SegmentsSlice<'a, TAnyBin> {
    type Item = BinSegment<'a, TAnyBin>;

    fn next(&mut self) -> Option<Self::Item> {
        let item = SegmentsSlice::take(self, self.pos);
        self.pos += 1;
        item
    }
}


impl<'a, TAnyBin: AnyBin> ExactSizeIterator for SegmentsSlice<'a, TAnyBin> {
    #[inline]
    fn len(&self) -> usize {
        self.remaining()
    }
}
