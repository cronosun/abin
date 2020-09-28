use std::ops::Deref;

pub trait AnyBin: Deref<Target=[u8]> {
    #[inline]
    fn as_slice(&self) -> &[u8] {
        self.deref()
    }
}