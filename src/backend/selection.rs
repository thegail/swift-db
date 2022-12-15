use crate::schema::Schema;

/// A pointer to a [`Document`] in the storage file.
///
/// Can be used in later [`Request`]s to the [`Backend`].
///
/// [`Document`]: crate::schema::Document
/// [`Request`]: crate::backend::Request
/// [`Backend`]: crate::backend::Backend
#[derive(Clone)]
pub struct Reference {
    /// The [`Schema`] describing the [`Document`] pointed
    /// to by this selection.
    ///
    /// [`Document`]: crate::schema::Document
    pub schema: Schema,
    /// The offset in the storage file at which the
    /// [`Document`] is stored.
    ///
    /// [`Document`]: crate::schema::Document
    pub(super) position: usize,
}

// A pointer to a list of [`Document`][crate::schema::Document]s.
//
// See [`Selection`].
// #[derive(Clone)]
// pub struct ManySelection {
//     pub schema: Schema,
//     pub(super) positions: Vec<usize>,
// }
