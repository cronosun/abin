use crate::{Bin, SyncBin};
use core::cell::RefCell;

std::thread_local! {
  static THREAD_LOCAL_BIN: RefCell<ThreadLocalValue> = RefCell::new(ThreadLocalValue::None);
}

fn TODO_demo() {
    THREAD_LOCAL_BIN.with(|x| {})
}

enum ThreadLocalValue {
    None,
    Bin(Bin),
    SyncBin(SyncBin),
}
