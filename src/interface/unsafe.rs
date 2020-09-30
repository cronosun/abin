use crate::{BinData, FnTable, SyncBin};

pub unsafe trait UnsafeBin {
    unsafe fn _new(data: BinData, fn_table: &'static FnTable) -> Self;
    unsafe fn _data(&self) -> &BinData;
    unsafe fn _data_mut(&mut self) -> &mut BinData;
    unsafe fn _fn_table(&self) -> &'static FnTable;
    unsafe fn _into_sync(self) -> SyncBin;
}
