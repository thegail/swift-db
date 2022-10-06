use crate::schema::{Document, FieldValue};

pub struct Query {
    pub collection: u64,
    pub fields_of_interest: Vec<u16>,
    pub condition: Condition,
}

pub enum Condition {
    Equal(Expression, Expression),
    NotEqual(Expression, Expression),
    GreaterThan(Expression, Expression),
    LessThan(Expression, Expression),
    Or(Box<Condition>, Box<Condition>),
    And(Box<Condition>, Box<Condition>),
    Not(Box<Condition>),
}

pub enum Expression {
    Value(FieldValue),
    Field(u16),
}

impl Document {
    fn eval_expr(&self, expr: Expression) -> Option<FieldValue> {
        match expr {
            Expression::Value(value) => Some(value),
            Expression::Field(field_id) => {
                if let Some(field_instance) = self.fields.iter().find(|x| x.id == field_id) {
                    Some(field_instance.value.clone())
                } else {
                    None
                }
            }
        }
    }
}

impl Condition {
    pub fn evaluate(self, doc: &Document) -> bool {
        match self {
            Condition::Equal(left, right) => {
                let left_value = doc.eval_expr(left).expect("TODO: condition error handling");
                let right_value = doc
                    .eval_expr(right)
                    .expect("TODO: condition error handling");
                match left_value {
                    FieldValue::Int(left_int) => {
                        if let FieldValue::Int(right_int) = right_value {
                            left_int == right_int
                        } else {
                            panic!("oh nooo")
                        }
                    }
                    FieldValue::UInt(left_uint) => {
                        if let FieldValue::UInt(right_uint) = right_value {
                            left_uint == right_uint
                        } else {
                            panic!("oh nooo")
                        }
                    }
                    FieldValue::Long(left_long) => {
                        if let FieldValue::Long(right_long) = right_value {
                            left_long == right_long
                        } else {
                            panic!("oh nooo")
                        }
                    }
                    FieldValue::ULong(left_ulong) => {
                        if let FieldValue::ULong(right_ulong) = right_value {
                            left_ulong == right_ulong
                        } else {
                            panic!("oh nooo")
                        }
                    }
                    FieldValue::Float(left_float) => {
                        if let FieldValue::Float(right_float) = right_value {
                            left_float == right_float
                        } else {
                            panic!("oh nooo")
                        }
                    }
                    FieldValue::Bool(left_bool) => {
                        if let FieldValue::Bool(right_bool) = right_value {
                            left_bool == right_bool
                        } else {
                            panic!("oh nooo")
                        }
                    }
                    FieldValue::DateTime(left_date) => {
                        if let FieldValue::DateTime(right_date) = right_value {
                            left_date == right_date
                        } else {
                            panic!("oh nooo")
                        }
                    }
                    FieldValue::String(left_string) => {
                        if let FieldValue::String(right_string) = right_value {
                            left_string == right_string
                        } else {
                            panic!("oh nooo")
                        }
                    }
                    FieldValue::ByteArray(left_bytes) => {
                        if let FieldValue::ByteArray(right_bytes) = right_value {
                            left_bytes == right_bytes
                        } else {
                            panic!("oh nooo")
                        }
                    }
                    FieldValue::Array(_) => {
                        panic!("oh nooo")
                    }
                    FieldValue::Object(_) => {
                        panic!("oh nooo")
                    }
                    FieldValue::Enum(_) => {
                        panic!("oh nooo")
                    }
                }
            }
            Condition::NotEqual(left, right) => {
                let left_value = doc.eval_expr(left).expect("TODO: condition error handling");
                let right_value = doc
                    .eval_expr(right)
                    .expect("TODO: condition error handling");
                match left_value {
                    FieldValue::Int(left_int) => {
                        if let FieldValue::Int(right_int) = right_value {
                            left_int != right_int
                        } else {
                            panic!("oh nooo")
                        }
                    }
                    FieldValue::UInt(left_uint) => {
                        if let FieldValue::UInt(right_uint) = right_value {
                            left_uint != right_uint
                        } else {
                            panic!("oh nooo")
                        }
                    }
                    FieldValue::Long(left_long) => {
                        if let FieldValue::Long(right_long) = right_value {
                            left_long != right_long
                        } else {
                            panic!("oh nooo")
                        }
                    }
                    FieldValue::ULong(left_ulong) => {
                        if let FieldValue::ULong(right_ulong) = right_value {
                            left_ulong != right_ulong
                        } else {
                            panic!("oh nooo")
                        }
                    }
                    FieldValue::Float(left_float) => {
                        if let FieldValue::Float(right_float) = right_value {
                            left_float != right_float
                        } else {
                            panic!("oh nooo")
                        }
                    }
                    FieldValue::Bool(left_bool) => {
                        if let FieldValue::Bool(right_bool) = right_value {
                            left_bool != right_bool
                        } else {
                            panic!("oh nooo")
                        }
                    }
                    FieldValue::DateTime(left_date) => {
                        if let FieldValue::DateTime(right_date) = right_value {
                            left_date != right_date
                        } else {
                            panic!("oh nooo")
                        }
                    }
                    FieldValue::String(left_string) => {
                        if let FieldValue::String(right_string) = right_value {
                            left_string != right_string
                        } else {
                            panic!("oh nooo")
                        }
                    }
                    FieldValue::ByteArray(left_bytes) => {
                        if let FieldValue::ByteArray(right_bytes) = right_value {
                            left_bytes != right_bytes
                        } else {
                            panic!("oh nooo")
                        }
                    }
                    FieldValue::Array(_) => {
                        panic!("oh nooo")
                    }
                    FieldValue::Object(_) => {
                        panic!("oh nooo")
                    }
                    FieldValue::Enum(_) => {
                        panic!("oh nooo")
                    }
                }
            }
            Condition::GreaterThan(left, right) => {
                let left_value = doc.eval_expr(left).expect("TODO: condition error handling");
                let right_value = doc
                    .eval_expr(right)
                    .expect("TODO: condition error handling");
                match left_value {
                    FieldValue::Int(left_int) => {
                        if let FieldValue::Int(right_int) = right_value {
                            left_int > right_int
                        } else {
                            panic!("oh nooo")
                        }
                    }
                    FieldValue::UInt(left_uint) => {
                        if let FieldValue::UInt(right_uint) = right_value {
                            left_uint > right_uint
                        } else {
                            panic!("oh nooo")
                        }
                    }
                    FieldValue::Long(left_long) => {
                        if let FieldValue::Long(right_long) = right_value {
                            left_long > right_long
                        } else {
                            panic!("oh nooo")
                        }
                    }
                    FieldValue::ULong(left_ulong) => {
                        if let FieldValue::ULong(right_ulong) = right_value {
                            left_ulong > right_ulong
                        } else {
                            panic!("oh nooo")
                        }
                    }
                    FieldValue::Float(left_float) => {
                        if let FieldValue::Float(right_float) = right_value {
                            left_float > right_float
                        } else {
                            panic!("oh nooo")
                        }
                    }
                    FieldValue::Bool(_) => {
                        panic!("oh nooo")
                    }
                    FieldValue::DateTime(left_date) => {
                        if let FieldValue::DateTime(right_date) = right_value {
                            left_date > right_date
                        } else {
                            panic!("oh nooo")
                        }
                    }
                    FieldValue::String(left_string) => {
                        if let FieldValue::String(right_string) = right_value {
                            left_string > right_string
                        } else {
                            panic!("oh nooo")
                        }
                    }
                    FieldValue::ByteArray(_) => {
                        panic!("oh nooo")
                    }
                    FieldValue::Array(_) => {
                        panic!("oh nooo")
                    }
                    FieldValue::Object(_) => {
                        panic!("oh nooo")
                    }
                    FieldValue::Enum(_) => {
                        panic!("oh nooo")
                    }
                }
            }
            Condition::LessThan(left, right) => {
                let left_value = doc.eval_expr(left).expect("TODO: condition error handling");
                let right_value = doc
                    .eval_expr(right)
                    .expect("TODO: condition error handling");
                match left_value {
                    FieldValue::Int(left_int) => {
                        if let FieldValue::Int(right_int) = right_value {
                            left_int < right_int
                        } else {
                            panic!("oh nooo")
                        }
                    }
                    FieldValue::UInt(left_uint) => {
                        if let FieldValue::UInt(right_uint) = right_value {
                            left_uint < right_uint
                        } else {
                            panic!("oh nooo")
                        }
                    }
                    FieldValue::Long(left_long) => {
                        if let FieldValue::Long(right_long) = right_value {
                            left_long < right_long
                        } else {
                            panic!("oh nooo")
                        }
                    }
                    FieldValue::ULong(left_ulong) => {
                        if let FieldValue::ULong(right_ulong) = right_value {
                            left_ulong < right_ulong
                        } else {
                            panic!("oh nooo")
                        }
                    }
                    FieldValue::Float(left_float) => {
                        if let FieldValue::Float(right_float) = right_value {
                            left_float < right_float
                        } else {
                            panic!("oh nooo")
                        }
                    }
                    FieldValue::Bool(_) => {
                        panic!("oh nooo")
                    }
                    FieldValue::DateTime(left_date) => {
                        if let FieldValue::DateTime(right_date) = right_value {
                            left_date < right_date
                        } else {
                            panic!("oh nooo")
                        }
                    }
                    FieldValue::String(left_string) => {
                        if let FieldValue::String(right_string) = right_value {
                            left_string < right_string
                        } else {
                            panic!("oh nooo")
                        }
                    }
                    FieldValue::ByteArray(_) => {
                        panic!("oh nooo")
                    }
                    FieldValue::Array(_) => {
                        panic!("oh nooo")
                    }
                    FieldValue::Object(_) => {
                        panic!("oh nooo")
                    }
                    FieldValue::Enum(_) => {
                        panic!("oh nooo")
                    }
                }
            }
            Condition::Or(left, right) => left.evaluate(doc) || right.evaluate(doc),
            Condition::And(left, right) => left.evaluate(doc) && right.evaluate(doc),
            Condition::Not(condition) => !condition.evaluate(doc),
        }
    }
}
