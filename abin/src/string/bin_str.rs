use crate::{AnyStr, Bin};

/// A string backed by `Bin` (not `Sync + Send`), see `SStr` if you need `Sync + Send`.
///
/// ```rust
/// use abin::{Str, NewStr, StrFactory};
/// let str : Str = NewStr::from_static("Hello");
/// assert_eq!("Hello", str.as_str());
/// ```
pub type Str = AnyStr<Bin>;
