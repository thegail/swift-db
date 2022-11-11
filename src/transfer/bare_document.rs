/// An intermediate format for serializing and deserializing
/// [`Document`]s.
///
/// Implements [`serde::Serialize`] and [`serde::Deserialize`].
///
/// [`Document`]: crate::schema::Document
pub struct BareDocument {
    pub fields: Vec<BareField>,
}

/// A member of a [`BareDocument`].
pub struct BareField {
    pub name: String,
    pub value: BareValue,
}

/// A value in a [`BareField`].
pub enum BareValue {
    Integer(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Array(Vec<BareValue>),
    Object(Box<BareDocument>),
}
