pub struct BinGen {
    seed: u8,
    len: usize,
}

impl BinGen {
    pub fn new(seed: u8, len: usize) -> Self {
        Self { seed, len }
    }

    pub fn generate(&self) -> impl Iterator<Item = u8> {
        let mut state = self.seed;
        (0..self.len).map(move |index| {
            state = state.wrapping_mul(31).wrapping_add(index as u8);
            state
        })
    }

    /// Generates vector. Capacity is undefined.
    pub fn generate_to_vec(&self) -> Vec<u8> {
        self.generate().collect()
    }

    /// Generates vector. Makes sure the vector has defined capacity.
    pub fn generate_to_vec_shrink(&self, additional_capacity: usize) -> Vec<u8> {
        let mut vec: Vec<u8> = self.generate_to_vec();
        let len = vec.len();
        vec.reserve(additional_capacity);
        unsafe {
            vec.set_len(len + additional_capacity);
        }
        vec.shrink_to_fit();
        unsafe {
            vec.set_len(len);
        }
        vec
    }
}
