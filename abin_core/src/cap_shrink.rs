pub trait VecCapShrink {
    fn is_shrink(len: usize, capacity: usize) -> bool;
    /// Do never shrink if vector has less or equal this capacity.
    fn min_capacity() -> usize;
}

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
