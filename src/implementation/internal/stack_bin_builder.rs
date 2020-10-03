use crate::{SBin, StackBin};

const MAX_LEN: usize = StackBin::max_len();

/// Unfortunately `SmallVec` does not implement the size for 23.
pub struct StackBinBuilder {
    vec_excess_capacity: usize,
    inner: Inner,
}

enum Inner {
    Vec(Vec<u8>),
    Stack { len: usize, array: [u8; MAX_LEN] },
}

impl StackBinBuilder {
    #[inline]
    pub fn new(vec_excess_capacity: usize) -> Self {
        Self {
            vec_excess_capacity,
            inner: Inner::Stack {
                len: 0,
                array: [0; MAX_LEN],
            },
        }
    }

    #[inline]
    pub fn extend_from_slice(&mut self, other: &[u8]) {
        match &mut self.inner {
            Inner::Vec(vec) => vec.extend_from_slice(other),
            Inner::Stack { len, array } => {
                let other_len = other.len();
                let resulting_len = len.checked_add(other_len).unwrap();
                if resulting_len > MAX_LEN {
                    // we need to use a vec
                    let mut vec = Vec::with_capacity(resulting_len + self.vec_excess_capacity);
                    vec.extend_from_slice(&array[0..*len]);
                    vec.extend_from_slice(other);
                    self.inner = Inner::Vec(vec);
                } else {
                    // ok, still enough for the stack
                    (&mut array[*len..]).copy_from_slice(other);
                    *len = resulting_len;
                }
            }
        }
    }

    pub fn build(self) -> Result<SBin, Vec<u8>> {
        match self.inner {
            Inner::Vec(vec) => {
                // too large
                Err(vec)
            }
            Inner::Stack { len, array } => Ok(StackBin::try_from(&array[0..len])
                .expect("This MUST be small enough for the stack.")),
        }
    }
}
