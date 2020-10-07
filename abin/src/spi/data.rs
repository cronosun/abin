/// This is just a placeholder for the payload of types. You only need this if you implement
/// your own type.
///
/// If you use your own bin data, make sure:
///  * The size must be exactly 3 words (3 * usize).
///  * The struct must be word-aligned (usize-aligned).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BinData(usize, usize, usize);

impl BinData {
    pub const fn empty() -> Self {
        Self(0, 0, 0)
    }
}
