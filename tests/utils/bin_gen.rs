pub struct BinGen {
    seed: u8,
    len: usize,
}

impl BinGen {
    pub fn new(seed: u8,
               len: usize) -> Self {
        Self { seed, len }
    }

    pub fn generate(&self) -> impl Iterator<Item=u8> {
        let mut state = self.seed;
        (0..self.len).map(move |index| {
            state = state.wrapping_mul(31).wrapping_add(index as u8);
            state
        })
    }

    pub fn generate_to_vec(&self) -> Vec<u8> {
        self.generate().collect()
    }
}

