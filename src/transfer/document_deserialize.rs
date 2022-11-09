use crate::schema::Document;
use serde::de::Visitor;
use serde::Deserialize;

struct BareDocument {
    fields: Vec<BareField>,
}

struct BareField {
    name: String,
    value: BareValue,
}

enum BareValue {
    Integer(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Array(Vec<BareValue>),
    Object(Box<BareDocument>),
    Enum(Box<BareEnumValue>),
}

struct BareEnumValue {
    case: String,
    associated_value: Option<BareValue>,
}

struct DocumentVisitor;

impl<'de> Visitor<'de> for DocumentVisitor {
    type Value = BareDocument;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(formatter, "a map of key-value pairs")
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut fields = Vec::with_capacity(map.size_hint().unwrap_or(0));
        while let Some((key, value)) = map.next_entry()? {
            fields.push(BareField {
                name: key,
                value: todo!(),
            });
        }
        let document = BareDocument { fields };
        Ok(document)
    }
}

impl<'de> Deserialize<'de> for BareDocument {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(DocumentVisitor)
    }
}
