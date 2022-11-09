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
}

struct DocumentVisitor;

impl<'de> Visitor<'de> for DocumentVisitor {
    type Value = BareDocument;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(formatter, "a map of key-value pairs")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut fields = Vec::with_capacity(map.size_hint().unwrap_or(0));
        while let Some((name, value)) = map.next_entry::<String, BareValue>()? {
            fields.push(BareField { name, value });
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

struct ValueVisitor;

impl<'de> Visitor<'de> for ValueVisitor {
    type Value = BareValue;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "any value")
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BareValue::Integer(v as i64))
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BareValue::Integer(v as i64))
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BareValue::Integer(v as i64))
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BareValue::Integer(v as i64))
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BareValue::Integer(v as i64))
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BareValue::Integer(v as i64))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BareValue::Integer(v))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v > i64::MAX as u64 {
            return Err(serde::de::Error::custom("u64 out of range"));
        }
        Ok(BareValue::Integer(v as i64))
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BareValue::Float(v as f64))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BareValue::Float(v))
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BareValue::Bool(v))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BareValue::String(v))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut values = Vec::with_capacity(seq.size_hint().unwrap_or(0));
        while let Some(value) = seq.next_element()? {
            values.push(value);
        }
        Ok(BareValue::Array(values))
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        Ok(BareValue::Object(Box::new(DocumentVisitor.visit_map(map)?)))
    }
}

impl<'de> Deserialize<'de> for BareValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(ValueVisitor)
    }
}
