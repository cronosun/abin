use crate::{AnyBin, AnyStr, StrSegment};

/// Trait used to build a string efficiently (with just one allocation & no re-allocation or
/// even without allocation).
///
/// ```rust
/// use abin::{NewStr, StrSegment, Str, StrBuilder};
///
/// let mut builder = NewStr::builder();
/// builder.push(StrSegment::Static("Hello, "));
/// builder.push(StrSegment::Static("World!"));
/// let string : Str = builder.build();
///
/// assert_eq!("Hello, World!", string.as_str());
/// ```
pub trait StrBuilder<'a> {
    /// The binary type the generated strings are backed by.
    type T: AnyBin;

    fn push(&mut self, segment: impl Into<StrSegment<'a, Self::T>>);

    #[inline]
    fn push_str(&mut self, string: impl Into<AnyStr<Self::T>>) {
        self.push(StrSegment::Str(string.into()));
    }

    #[inline]
    fn push_slice(&mut self, string: impl Into<&'a str>) {
        self.push(StrSegment::Slice(string.into()));
    }

    #[inline]
    fn push_static(&mut self, slice: impl Into<&'static str>) {
        self.push(StrSegment::Static(slice.into()));
    }

    #[inline]
    fn push_given_string(&mut self, string: impl Into<String>) {
        self.push(StrSegment::GivenString(string.into()));
    }

    #[inline]
    fn push_char(&mut self, chr: char) {
        self.push(StrSegment::Char(chr));
    }

    /// Builds the string.
    ///
    /// Note: After calling this method, the builder will be empty again and can be re-used. We
    /// use `&mut self` here instead of `self` to make sure the builder is not copied (it's large).
    /// I'm not sure how well rust would optimize `self` here.
    fn build(&mut self) -> AnyStr<Self::T>;
}
