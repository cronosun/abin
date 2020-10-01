use std::cmp::max;

/// Gives information whether the system should shrink a vector.
pub trait VecCapShrink {
    /// Returns a result whether the vector should be shrunk.
    ///
    /// `excess`: `Vec::len() - Vec::capacity()`.
    fn shrink(len: usize, excess: usize) -> ShrinkResult;
}

/// Default implementation of `VecCapShrink` - should be ok for most use cases.
///
/// Tolerated excess:
///
/// * 48 bytes for tiny vectors below 1k len.
/// * 256 bytes for 1k-4k len vectors;
/// * 1kb for 4k - 32k len vectors;
/// * 4kb for for 32k+ len vectors;
pub struct DefaultVecCapShrink;

impl VecCapShrink for DefaultVecCapShrink {
    #[inline]
    fn shrink(len: usize, excess: usize) -> ShrinkResult {
        if excess < 48 + 1 {
            // fast path: 99% case
            ShrinkResult::do_not_shrink()
        } else {
            if len < 1024 + 1 {
                // < 1kb
                ShrinkResult::shrink_with_remaining_excess(48)
            } else if len < 1024 * 4 + 1 {
                // 1kb - 4kb
                if excess > 256 {
                    ShrinkResult::shrink_with_remaining_excess(64)
                } else {
                    ShrinkResult::do_not_shrink()
                }
            } else if len < 1024 * 32 + 1 {
                // 4kb - 32kb
                if excess > 1024 * 1 {
                    ShrinkResult::shrink_with_remaining_excess(128)
                } else {
                    ShrinkResult::do_not_shrink()
                }
            } else {
                // large
                if excess > 1024 * 4 + 1 {
                    ShrinkResult::shrink_with_remaining_excess(256)
                } else {
                    ShrinkResult::do_not_shrink()
                }
            }
        }
    }
}

/// Never performs a capacity shrink.
pub struct NoVecCapShrink;

impl VecCapShrink for NoVecCapShrink {
    fn shrink(_len: usize, _capacity: usize) -> ShrinkResult {
        ShrinkResult::do_not_shrink()
    }
}

/// Whether to shrink the vector and if so, how much excess to keep.
#[derive(Debug, Copy, Clone)]
pub struct ShrinkResult(u32);

impl ShrinkResult {
    /// Keep the vector untouched (do not shrink).
    #[inline]
    pub fn do_not_shrink() -> Self {
        Self(core::u32::MAX)
    }

    /// Shrinks the vector but keeps some remaining excess.
    #[inline]
    pub fn shrink_with_remaining_excess(remaining_excess: u16) -> Self {
        Self(remaining_excess as u32)
    }

    #[inline]
    fn do_shrink(self) -> bool {
        self.0 != core::u32::MAX
    }
}

/// Shrinks the vector (if T says to do so). Returns true if vec has been shrunk,
/// returns false if not.
///
/// `never_below_excess`: Makes sure to never shrink the vector below given excess.
#[inline]
pub(crate) fn maybe_shrink_vec<T: VecCapShrink>(
    vec: &mut Vec<u8>,
    never_below_excess: usize,
) -> bool {
    let len = vec.len();
    let capacity = vec.capacity();
    let excess = capacity - len;
    let result = T::shrink(len, excess);
    if result.do_shrink() {
        let excess_to_keep = result.0 as usize;
        let excess_to_keep = max(excess_to_keep, never_below_excess);
        if excess > excess_to_keep {
            // ok, here we shrink the vector.
            unsafe {
                vec.set_len(len + excess_to_keep);
            }
            vec.shrink_to_fit();
            unsafe {
                vec.set_len(len);
            }
            true
        } else {
            // still nothing to do
            false
        }
    } else {
        // nothing to shrink
        false
    }
}
