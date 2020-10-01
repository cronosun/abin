/// An iterator that can be used for `AnyRc::from_iter` that guarantees one single allocation
/// when chaining multiple slices into one binary (can also be used for other things, like for
/// constructing a `Vec`).
///
/// ```rust
/// use abin::{StaticBin, EmptyBin, RcBin, AnyRc, ChainSlicesIter, AnyBin};
///
/// let bin1 = StaticBin::from("static value, ".as_bytes());
/// let bin2 = EmptyBin::new();
/// let bin3 = RcBin::copy_from_slice("another binary".as_bytes());
///
/// // the rest of the code needs just one single allocation (RcBin::from_iter allocates).
/// let slice = [bin1.as_slice(), bin2.as_slice(), bin3.as_slice()];
/// let chain_slices = ChainSlicesIter::from(&slice as &[&[u8]]);
/// assert_eq!(28, chain_slices.len());
/// assert_eq!((28, Some(28)), chain_slices.size_hint());
/// let chained = RcBin::from_iter(chain_slices);
/// assert_eq!("static value, another binary".as_bytes(), chained.as_slice());
/// ```
pub struct ChainSlicesIter<'a> {
    slices: &'a [&'a [u8]],
    len: usize,
    pos: usize,
    slice_index: usize,
    in_slice_index: usize,
}

impl<'a> ChainSlicesIter<'a> {
    #[inline]
    pub fn new(slices: &'a [&'a [u8]]) -> Self {
        // compute the entire length
        let len = slices.iter().map(|item| item.len()).sum();
        Self {
            slices,
            len,
            pos: 0,
            slice_index: 0,
            in_slice_index: 0,
        }
    }

    #[inline]
    fn remaining(&self) -> usize {
        if self.pos < self.len {
            self.len - self.pos
        } else {
            0
        }
    }
}

impl<'a> From<&'a [&'a [u8]]> for ChainSlicesIter<'a> {
    #[inline]
    fn from(slices: &'a [&'a [u8]]) -> Self {
        ChainSlicesIter::new(slices)
    }
}

impl<'a> Iterator for ChainSlicesIter<'a> {
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let current_pos = self.pos;
        if current_pos < self.len {
            // we always must have a slice
            let current_slice = self
                .slices
                .get(self.slice_index)
                .expect("Must always have a slice as long as current_pos < self.len");
            if let Some(value) = current_slice.get(self.in_slice_index) {
                // current slice still has items.
                self.in_slice_index += 1;
                self.pos += 1;
                Some(*value)
            } else {
                // no value, go to next slice.
                self.slice_index += 1;
                self.in_slice_index = 0;
                self.next()
            }
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let rem = self.remaining();
        (rem, Some(rem))
    }
}

impl<'a> ExactSizeIterator for ChainSlicesIter<'a> {
    #[inline]
    fn len(&self) -> usize {
        self.remaining()
    }
}
