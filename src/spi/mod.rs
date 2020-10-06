//! The SPI (Service Provider Interface) contains types you only need if you want to provide
//! your own binary implementation. Most types/functions in this module are unsafe. If you're
//! using things from this module or if you need `unsafe`, this means two things: You're either
//! implementing your own binary type or you're doing something wrong (as a user of this crate
//! you won't need unsafe code, not things from this module).

pub use {data::*, fn_table::*, r#unsafe::*};

mod data;
mod fn_table;
mod r#unsafe;
