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

impl<'de> Deserialize<'de> for BareDocument {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_bytes(visitor)
    }
}
