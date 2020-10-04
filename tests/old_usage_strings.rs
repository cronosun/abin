use std::ops::Deref;

use abin::{NewSStr, NewStr, SStr, Str, StrFactory};

#[test]
fn create_strings() {
    // no allocation
    let _str_static = NewStr::from_static("Static string");
    let _sync_static = NewSStr::from_static("A static synchronized string");

    let non_static = "small".to_owned();

    // small strings (no allocation; stack only).
    let small = NewStr::copy_from_str(non_static.as_str());
    let sync_small = NewSStr::from_given_string(non_static);

    let non_static_bigger = "This is some bigger string that does not fit onto the stack.";
    // this allocates.
    let _from_slice = NewStr::copy_from_str(non_static_bigger.deref());
    // or better like this (if there's enough capacity, this can prevent allocation):
    let _from_string = NewStr::from_static(non_static_bigger);
}
