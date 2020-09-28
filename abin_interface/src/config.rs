use crate::Bin;

pub struct BinConfig {
    pub drop: fn(bin: &mut Bin),
    pub as_slice: fn(bin: &Bin) -> &[u8],
    pub is_empty: fn(bin: &Bin) -> bool,
    pub clone: fn(bin: &Bin) -> Bin,
    pub into_vec: fn(bin: Bin) -> Vec<u8>,
}