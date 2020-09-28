use std::ops::Deref;

pub trait AnyBin: Deref<Target=[u8]> + Clone {
    #[inline]
    fn as_slice(&self) -> &[u8] {
        self.deref()
    }
}