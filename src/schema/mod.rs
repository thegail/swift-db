//! The `schema` module defines both the definitions
//! and instances of stored values.
//!
//! | Definition    | Instance          |
//! |---------------|-------------------|
//! | [`Schema`]    | [`Document`]      |
//! | [`Field`]     | [`FieldInstance`] |
//! | [`FieldType`] | [`FieldValue`]    |
//! | [`EnumCase`]  | [`EnumValue`]     |
mod document;
mod field;
mod field_instance;
mod field_type;
mod field_value;
#[allow(clippy::module_inception)]
mod schema;

pub use document::Document;
pub use field::Field;
pub use field_instance::FieldInstance;
pub use field_type::{EnumCase, FieldType};
pub use field_value::{EnumValue, FieldValue};
pub use schema::Schema;
