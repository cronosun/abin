use crate::AnyBin;

/// Consuming iterator for a binary.
#[derive(Debug)]
pub struct IntoIter<T> {
    pos: usize,
    inner: T,
    len: usize,
}

impl<T: AnyBin> IntoIter<T> {
    /// Creates an iterator over the bytes contained by the binary, starting at given
    /// position (`pos`). The position is usually `0` (if you want to read the binary from
    /// the start).
    pub fn new(inner: T, pos: usize) -> Self {
        let len = inner.len();
        Self { inner, pos, len }
    }

    /// Consumes this `IntoIter`, returning the underlying value.
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T: AnyBin> Iterator for IntoIter<T> {
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<u8> {
        let current_pos = self.pos;
        if current_pos < self.len {
            self.pos += 1;
            Some(self.inner.as_slice()[current_pos])
        } else {
            // out of bounds
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let rem = if self.pos < self.len {
            self.len - self.pos
        } else {
            0
        };
        (rem, Some(rem))
    }
}

impl<T: AnyBin> ExactSizeIterator for IntoIter<T> {}
