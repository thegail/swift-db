use crate::schema::{Document, Field, FieldType, FieldValue};
use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize, Serializer};
use std::io::Write;

impl Serialize for Document {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let map = serializer.serialize_map(Some(self.fields.len()))?;
        for field in self.fields {
            field
        }
        map.end()
    }
}
