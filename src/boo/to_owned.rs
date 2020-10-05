/// Converts the borrowed value of `Boo` to owned.
pub trait BooToOwned<TBorrowed : ?Sized, TOwned> {
    fn convert_to_owned(borrowed: &TBorrowed) -> TOwned;
}