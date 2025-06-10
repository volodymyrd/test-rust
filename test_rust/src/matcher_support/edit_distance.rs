/// Controls the termination condition of [`edit_list`].
#[derive(Clone, Copy)]
pub(crate) enum Mode {
    /// Indicates that the two arguments are intended to be equal.
    ///
    /// The entire edit list to transform between `actual` and `expected` is
    /// returned.
    Exact,
}
