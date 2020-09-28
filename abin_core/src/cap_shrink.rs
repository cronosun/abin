/// Gives information whether the system should shrink a vector before using it as `Bin`.
pub trait VecCapShrink {
    /// Returns `true` if the vector should be shrunk.
    fn is_shrink(len: usize, capacity: usize) -> bool;
    /// Do never shrink if vector has less or equal this capacity.
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
            too_much > 1024 * 8
        } else {
            // large
            too_much > 1024 * 32
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
    fn is_shrink(len: usize, capacity: usize) -> bool {
        false
    }

    #[inline]
    fn min_capacity() -> usize {
        std::usize::MAX
    }
}