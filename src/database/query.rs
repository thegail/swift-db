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
    fn eval_expr<'a>(&'a self, expr: &'a Expression) -> Option<&'a FieldValue> {
        // Holy shit using named lifetimes actually worked !! --tg, beginning Rust programmer
        match expr {
            Expression::Value(value) => Some(value),
            Expression::Field(field_id) => {
                if let Some(field_instance) = self.fields.iter().find(|x| x.id == *field_id) {
                    Some(&field_instance.value)
                } else {
                    None
                }
            }
        }
    }

    pub fn evaluate(&self, condition: &Condition) -> bool {
        match condition {
            Condition::Equal(left, right) => {
                let left_value = self
                    .eval_expr(&left)
                    .expect("TODO: condition error handling");
                let right_value = self
                    .eval_expr(&right)
                    .expect("TODO: condition error handling");
                match left_value {
                    FieldValue::Int(left_int) => {
                        if let FieldValue::Int(right_int) = right_value {
                            left_int == right_int
                        } else {
                            todo!()
                        }
                    }
                    FieldValue::UInt(left_uint) => {
                        if let FieldValue::UInt(right_uint) = right_value {
                            left_uint == right_uint
                        } else {
                            todo!()
                        }
                    }
                    FieldValue::Long(left_long) => {
                        if let FieldValue::Long(right_long) = right_value {
                            left_long == right_long
                        } else {
                            todo!()
                        }
                    }
                    FieldValue::ULong(left_ulong) => {
                        if let FieldValue::ULong(right_ulong) = right_value {
                            left_ulong == right_ulong
                        } else {
                            todo!()
                        }
                    }
                    FieldValue::Float(left_float) => {
                        if let FieldValue::Float(right_float) = right_value {
                            left_float == right_float
                        } else {
                            todo!()
                        }
                    }
                    FieldValue::Bool(left_bool) => {
                        if let FieldValue::Bool(right_bool) = right_value {
                            left_bool == right_bool
                        } else {
                            todo!()
                        }
                    }
                    FieldValue::DateTime(left_date) => {
                        if let FieldValue::DateTime(right_date) = right_value {
                            left_date == right_date
                        } else {
                            todo!()
                        }
                    }
                    FieldValue::String(left_string) => {
                        if let FieldValue::String(right_string) = right_value {
                            left_string == right_string
                        } else {
                            todo!()
                        }
                    }
                    FieldValue::ByteArray(left_bytes) => {
                        if let FieldValue::ByteArray(right_bytes) = right_value {
                            left_bytes == right_bytes
                        } else {
                            todo!()
                        }
                    }
                    FieldValue::Array(_) => {
                        todo!()
                    }
                    FieldValue::Object(_) => {
                        todo!()
                    }
                    FieldValue::Enum(_) => {
                        todo!()
                    }
                }
            }
            Condition::NotEqual(left, right) => {
                let left_value = self
                    .eval_expr(&left)
                    .expect("TODO: condition error handling");
                let right_value = self
                    .eval_expr(&right)
                    .expect("TODO: condition error handling");
                match left_value {
                    FieldValue::Int(left_int) => {
                        if let FieldValue::Int(right_int) = right_value {
                            left_int != right_int
                        } else {
                            todo!()
                        }
                    }
                    FieldValue::UInt(left_uint) => {
                        if let FieldValue::UInt(right_uint) = right_value {
                            left_uint != right_uint
                        } else {
                            todo!()
                        }
                    }
                    FieldValue::Long(left_long) => {
                        if let FieldValue::Long(right_long) = right_value {
                            left_long != right_long
                        } else {
                            todo!()
                        }
                    }
                    FieldValue::ULong(left_ulong) => {
                        if let FieldValue::ULong(right_ulong) = right_value {
                            left_ulong != right_ulong
                        } else {
                            todo!()
                        }
                    }
                    FieldValue::Float(left_float) => {
                        if let FieldValue::Float(right_float) = right_value {
                            left_float != right_float
                        } else {
                            todo!()
                        }
                    }
                    FieldValue::Bool(left_bool) => {
                        if let FieldValue::Bool(right_bool) = right_value {
                            left_bool != right_bool
                        } else {
                            todo!()
                        }
                    }
                    FieldValue::DateTime(left_date) => {
                        if let FieldValue::DateTime(right_date) = right_value {
                            left_date != right_date
                        } else {
                            todo!()
                        }
                    }
                    FieldValue::String(left_string) => {
                        if let FieldValue::String(right_string) = right_value {
                            left_string != right_string
                        } else {
                            todo!()
                        }
                    }
                    FieldValue::ByteArray(left_bytes) => {
                        if let FieldValue::ByteArray(right_bytes) = right_value {
                            left_bytes != right_bytes
                        } else {
                            todo!()
                        }
                    }
                    FieldValue::Array(_) => {
                        todo!()
                    }
                    FieldValue::Object(_) => {
                        todo!()
                    }
                    FieldValue::Enum(_) => {
                        todo!()
                    }
                }
            }
            Condition::GreaterThan(left, right) => {
                let left_value = self
                    .eval_expr(&left)
                    .expect("TODO: condition error handling");
                let right_value = self
                    .eval_expr(&right)
                    .expect("TODO: condition error handling");
                match left_value {
                    FieldValue::Int(left_int) => {
                        if let FieldValue::Int(right_int) = right_value {
                            left_int > right_int
                        } else {
                            todo!()
                        }
                    }
                    FieldValue::UInt(left_uint) => {
                        if let FieldValue::UInt(right_uint) = right_value {
                            left_uint > right_uint
                        } else {
                            todo!()
                        }
                    }
                    FieldValue::Long(left_long) => {
                        if let FieldValue::Long(right_long) = right_value {
                            left_long > right_long
                        } else {
                            todo!()
                        }
                    }
                    FieldValue::ULong(left_ulong) => {
                        if let FieldValue::ULong(right_ulong) = right_value {
                            left_ulong > right_ulong
                        } else {
                            todo!()
                        }
                    }
                    FieldValue::Float(left_float) => {
                        if let FieldValue::Float(right_float) = right_value {
                            left_float > right_float
                        } else {
                            todo!()
                        }
                    }
                    FieldValue::Bool(_) => {
                        todo!()
                    }
                    FieldValue::DateTime(left_date) => {
                        if let FieldValue::DateTime(right_date) = right_value {
                            left_date > right_date
                        } else {
                            todo!()
                        }
                    }
                    FieldValue::String(left_string) => {
                        if let FieldValue::String(right_string) = right_value {
                            left_string > right_string
                        } else {
                            todo!()
                        }
                    }
                    FieldValue::ByteArray(_) => {
                        todo!()
                    }
                    FieldValue::Array(_) => {
                        todo!()
                    }
                    FieldValue::Object(_) => {
                        todo!()
                    }
                    FieldValue::Enum(_) => {
                        todo!()
                    }
                }
            }
            Condition::LessThan(left, right) => {
                let left_value = self
                    .eval_expr(&left)
                    .expect("TODO: condition error handling");
                let right_value = self
                    .eval_expr(&right)
                    .expect("TODO: condition error handling");
                match left_value {
                    FieldValue::Int(left_int) => {
                        if let FieldValue::Int(right_int) = right_value {
                            left_int < right_int
                        } else {
                            todo!()
                        }
                    }
                    FieldValue::UInt(left_uint) => {
                        if let FieldValue::UInt(right_uint) = right_value {
                            left_uint < right_uint
                        } else {
                            todo!()
                        }
                    }
                    FieldValue::Long(left_long) => {
                        if let FieldValue::Long(right_long) = right_value {
                            left_long < right_long
                        } else {
                            todo!()
                        }
                    }
                    FieldValue::ULong(left_ulong) => {
                        if let FieldValue::ULong(right_ulong) = right_value {
                            left_ulong < right_ulong
                        } else {
                            todo!()
                        }
                    }
                    FieldValue::Float(left_float) => {
                        if let FieldValue::Float(right_float) = right_value {
                            left_float < right_float
                        } else {
                            todo!()
                        }
                    }
                    FieldValue::Bool(_) => {
                        todo!()
                    }
                    FieldValue::DateTime(left_date) => {
                        if let FieldValue::DateTime(right_date) = right_value {
                            left_date < right_date
                        } else {
                            todo!()
                        }
                    }
                    FieldValue::String(left_string) => {
                        if let FieldValue::String(right_string) = right_value {
                            left_string < right_string
                        } else {
                            todo!()
                        }
                    }
                    FieldValue::ByteArray(_) => {
                        todo!()
                    }
                    FieldValue::Array(_) => {
                        todo!()
                    }
                    FieldValue::Object(_) => {
                        todo!()
                    }
                    FieldValue::Enum(_) => {
                        todo!()
                    }
                }
            }
            Condition::Or(left, right) => self.evaluate(left) || self.evaluate(right),
            Condition::And(left, right) => self.evaluate(left) && self.evaluate(right),
            Condition::Not(condition) => !self.evaluate(condition),
        }
    }
}
