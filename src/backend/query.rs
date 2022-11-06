use super::operation_error::OperationError;
use crate::schema::{Document, FieldType, FieldValue};

pub struct Query {
    pub collection: u64,
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

macro_rules! eval_match_arm {
    ($i:ident, $l:expr, $r: expr, $o: tt) => {
        if let FieldValue::$i(right_unwrapped) = $r {
            Ok($l $o right_unwrapped)
        } else {
            Err(OperationError::ExpressionTypeMismatch {
                left: FieldType::$i,
                right: $r.simple_type(),
            })
        }
    };
}

impl Document {
    fn eval_expr<'a>(&'a self, expr: &'a Expression) -> Result<&'a FieldValue, OperationError> {
        match expr {
            Expression::Value(value) => Ok(value),
            Expression::Field(field_id) => {
                if let Some(field_instance) = self.fields.iter().find(|x| x.id == *field_id) {
                    Ok(&field_instance.value)
                } else {
                    Err(OperationError::UnknownFieldIdentifier)
                }
            }
        }
    }

    pub fn evaluate(&self, condition: &Condition) -> Result<bool, OperationError> {
        match condition {
            Condition::Equal(left, right) => {
                let left_value = self.eval_expr(left)?;
                let right_value = self.eval_expr(right)?;
                let r = right_value;
                match left_value {
                    FieldValue::Int(l) => eval_match_arm!(Int, l, r, ==),
                    FieldValue::UInt(l) => eval_match_arm!(UInt, l, r, ==),
                    FieldValue::Long(l) => eval_match_arm!(Long, l, r, ==),
                    FieldValue::ULong(l) => eval_match_arm!(ULong, l, r, ==),
                    FieldValue::Float(l) => eval_match_arm!(Float, l, r, ==),
                    FieldValue::Bool(l) => eval_match_arm!(Bool, l, r, ==),
                    FieldValue::DateTime(l) => eval_match_arm!(DateTime, l, r, ==),
                    FieldValue::String(l) => eval_match_arm!(String, l, r, ==),
                    FieldValue::ByteArray(l) => eval_match_arm!(ByteArray, l, r, ==),
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
                let left_value = self.eval_expr(left)?;
                let right_value = self.eval_expr(right)?;
                let r = right_value;
                match left_value {
                    FieldValue::Int(l) => eval_match_arm!(Int, l, r, !=),
                    FieldValue::UInt(l) => eval_match_arm!(UInt, l, r, !=),
                    FieldValue::Long(l) => eval_match_arm!(Long, l, r, !=),
                    FieldValue::ULong(l) => eval_match_arm!(ULong, l, r, !=),
                    FieldValue::Float(l) => eval_match_arm!(Float, l, r, !=),
                    FieldValue::Bool(l) => eval_match_arm!(Bool, l, r, !=),
                    FieldValue::DateTime(l) => eval_match_arm!(DateTime, l, r, !=),
                    FieldValue::String(l) => eval_match_arm!(String, l, r, !=),
                    FieldValue::ByteArray(l) => eval_match_arm!(ByteArray, l, r, !=),
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
                let left_value = self.eval_expr(left)?;
                let right_value = self.eval_expr(right)?;
                let r = right_value;
                match left_value {
                    FieldValue::Int(l) => eval_match_arm!(Int, l, r, >),
                    FieldValue::UInt(l) => eval_match_arm!(UInt, l, r, >),
                    FieldValue::Long(l) => eval_match_arm!(Long, l, r, >),
                    FieldValue::ULong(l) => eval_match_arm!(ULong, l, r, >),
                    FieldValue::Float(l) => eval_match_arm!(Float, l, r, >),
                    FieldValue::Bool(_) => todo!(),
                    FieldValue::DateTime(l) => eval_match_arm!(DateTime, l, r, >),
                    FieldValue::String(l) => eval_match_arm!(String, l, r, >),
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
                let left_value = self.eval_expr(left)?;
                let right_value = self.eval_expr(right)?;
                let r = right_value;
                match left_value {
                    FieldValue::Int(l) => eval_match_arm!(Int, l, r, <),
                    FieldValue::UInt(l) => eval_match_arm!(UInt, l, r, <),
                    FieldValue::Long(l) => eval_match_arm!(Long, l, r, <),
                    FieldValue::ULong(l) => eval_match_arm!(ULong, l, r, <),
                    FieldValue::Float(l) => eval_match_arm!(Float, l, r, <),
                    FieldValue::Bool(_) => todo!(),
                    FieldValue::DateTime(l) => eval_match_arm!(DateTime, l, r, <),
                    FieldValue::String(l) => eval_match_arm!(String, l, r, <),
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
            Condition::Or(left, right) => Ok(self.evaluate(left)? || self.evaluate(right)?),
            Condition::And(left, right) => Ok(self.evaluate(left)? && self.evaluate(right)?),
            Condition::Not(condition) => Ok(!self.evaluate(condition)?),
        }
    }
}
