use crate::{AnyStr, SBin};

/// A string backed by `SBin` (`Sync + Send`), see `Str` if you don't need `Sync + Send`.
///
/// ```rust
/// use abin::{SStr, NewSStr, StrFactory};
/// let str : SStr = NewSStr::from_static("Hello");
/// assert_eq!("Hello", str.as_str());
/// ```
pub type SStr = AnyStr<SBin>;
