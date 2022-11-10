pub struct BareDocument {
    pub fields: Vec<BareField>,
}

pub struct BareField {
    pub name: String,
    pub value: BareValue,
}

pub enum BareValue {
    Integer(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Array(Vec<BareValue>),
    Object(Box<BareDocument>),
}
