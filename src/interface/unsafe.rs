use crate::{BinData, FnTable, SBin};

/// Unsafe interface for [Bin](struct.Bin.html). This is only to be used if you want to
/// implement your own binary type.
pub unsafe trait UnsafeBin {
    unsafe fn _new(data: BinData, fn_table: &'static FnTable) -> Self;
    unsafe fn _data(&self) -> &BinData;
    unsafe fn _data_mut(&mut self) -> &mut BinData;
    unsafe fn _fn_table(&self) -> &'static FnTable;
    unsafe fn _into_sync(self) -> SBin;
}
