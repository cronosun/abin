use crate::{BinData, BinConfig, SyncBin};

pub unsafe trait UnsafeBin {
    fn _new(data : BinData, config : &'static BinConfig) -> Self;
    fn _data(&self) -> &BinData;
    fn _data_mut(&mut self) -> &mut BinData;
    fn _config(&self) -> &'static BinConfig;
    fn _into_sync(self) -> SyncBin;
}