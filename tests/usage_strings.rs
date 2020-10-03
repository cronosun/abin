use std::ops::Deref;

use abin::{Str, SyncStr};

#[test]
fn create_strings() {
    // no allocation
    let str_static = Str::from_static("Static string");
    let sync_static = SyncStr::from_static("A static synchronized string");

    let non_static = "small".to_owned();

    // small strings (no allocation; stack only).
    let small = Str::from(non_static.as_str());
    let sync_small = SyncStr::from(non_static);

    let non_static_bigger = "This is some bigger string that does not fit onto the stack.";
    // this allocates.
    let from_slice = Str::from(non_static_bigger.deref());
    // or better like this (if there's enough capacity, this can prevent allocation):
    let from_string = Str::from(non_static_bigger);
}
