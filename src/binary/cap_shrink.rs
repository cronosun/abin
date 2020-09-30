// TODO: Improve this: Remove `min_capacity` and is_shrink should give hints on remaining capacity...

#[inline]
pub(crate) fn is_shrink<T: VecCapShrink>(len: usize, capacity: usize) -> bool {
    if capacity > T::min_capacity() {
        T::is_shrink(len, capacity)
    } else {
        false
    }
}

/// Gives information whether the system should shrink a vector.
pub trait VecCapShrink {
    /// Returns `true` if the vector should be shrunk.
    fn is_shrink(len: usize, capacity: usize) -> bool;
    /// Do never shrink if vector has less or equal this capacity (fast fail for small allocations).
    fn min_capacity() -> usize;
}

/// Default implementation of `VecCapShrink` - should be ok for most use cases.
pub struct DefaultVecCapShrink;

impl VecCapShrink for DefaultVecCapShrink {
    #[inline]
    fn is_shrink(len: usize, capacity: usize) -> bool {
        let too_much = capacity - len;
        if len < 1024 {
            // small vectors
            too_much > 512
        } else if len < 1024 * 32 {
            // medium sized
            too_much > 1024 * 4
        } else {
            // large
            too_much > 1024 * 8
        }
    }

    #[inline]
    fn min_capacity() -> usize {
        255
    }
}

/// Never performs a capacity shrink.
pub struct NoVecCapShrink;

impl VecCapShrink for NoVecCapShrink {
    #[inline]
    fn is_shrink(_len: usize, _capacity: usize) -> bool {
        false
    }

    #[inline]
    fn min_capacity() -> usize {
        std::usize::MAX
    }
}
