/// This can be used in `Vec::from_iter` to create a vector that has more capacity than
/// the `inner` iterator needs.
pub struct SizeHintExtendingIter<TInner> {
    inner: TInner,
    extend_size_by: usize,
}

impl<TInner> SizeHintExtendingIter<TInner> {
    pub fn new(inner: TInner, extend_size_by: usize) -> Self {
        Self {
            inner,
            extend_size_by,
        }
    }
}

impl<TInner> Iterator for SizeHintExtendingIter<TInner>
where
    TInner: Iterator,
{
    type Item = TInner::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    /// Ok, manipulating this is maybe not very nice but it seems to be OK when used in
    /// `Vec::from_iter`; according to the documentation no bad things (like memory corruption)
    /// should ever happen (even if the `Vec::from_iter` implementation changes):
    ///
    /// > `size_hint()` is primarily intended to be used for optimizations such as
    /// > reserving space for the elements of the iterator, but must not be
    /// > trusted to e.g., omit bounds checks in unsafe code. An incorrect
    /// > implementation of `size_hint()` should not lead to memory safety
    /// > violations.
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.inner.size_hint();
        let upper = upper.map(|upper| upper.saturating_add(self.extend_size_by));
        let lower = lower.saturating_add(self.extend_size_by);
        (lower, upper)
    }
}

impl<TInner> ExactSizeIterator for SizeHintExtendingIter<TInner>
where
    TInner: ExactSizeIterator,
{
    /// here we return the correct length.
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}
