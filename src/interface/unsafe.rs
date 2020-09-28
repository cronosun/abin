use crate::{BinData, BinConfig, SyncBin};

pub unsafe trait UnsafeBin {
    unsafe fn _new(data : BinData, config : &'static BinConfig) -> Self;
    unsafe fn _data(&self) -> &BinData;
    unsafe fn _data_mut(&mut self) -> &mut BinData;
    unsafe fn _config(&self) -> &'static BinConfig;
    unsafe fn _into_sync(self) -> SyncBin;
}