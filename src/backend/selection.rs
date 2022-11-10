use crate::schema::Schema;

/// A pointer to a [`Document`][crate::schema::Document]
/// in the storage file.
///
/// Can be used in later [`Request`][crate::backend::Request]s
/// to the [`Backend`][crate::backend::Backend].
#[derive(Clone)]
pub struct Selection {
    /// The [`Schema`] describing the
    /// [`Document`][crate::schema::Document] pointed to by
    /// this selection.
    pub schema: Schema,
    /// The offset in the storage file at which the
    /// [`Document`][crate::schema::Document] is stored.
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
