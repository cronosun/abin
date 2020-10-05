/// Converts the borrowed value of `Boo` to owned. See `Boo::into_owned_with` for details.
pub trait BooToOwned<TBorrowed: ?Sized, TOwned> {
    /// Converts the borrowed value of `Boo` to owned.
    fn convert_to_owned(borrowed: &TBorrowed) -> TOwned;
}
