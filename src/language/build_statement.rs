use super::expression::Expression;
use crate::backend::{Condition, Expression as ValueExpression, Query, Selection};
use crate::language::{ParseError, Statement};
use crate::schema::{Document, FieldValue, Schema};
use std::collections::HashMap;
use std::io::Read;

/// Builds a [`Statement`] from a parsed expression.
///
/// Also takes schema information to correctly parse collection and field
/// references, and a [`Read`] in order to parse additional information,
/// such as document JSON.
pub fn build_statement(
    expression: &[Expression],
    collections: &[Schema],
    selections: HashMap<String, &Selection>,
    reader: impl Read,
) -> Result<Statement, ParseError> {
    let keyword = expression
        .first()
        .ok_or(ParseError::ArgumentCount)?
        .get_identifier()?;
    match keyword.as_str() {
        "open" => build_open(expression),
        "acquire" => build_acquire(expression),
        "commit" => build_commit(expression),
        "close" => build_close(expression),
        "select" => build_select(expression, collections),
        "create" => build_create(expression, collections, reader),
        "readall" => build_read_all(expression),
        "updateall" => build_update_all(expression, selections, reader),
        "delete" => build_delete(expression),
        _ => Err(ParseError::UnexpectedToken),
    }
}

fn build_open(expression: &[Expression]) -> Result<Statement, ParseError> {
    if expression.len() != 2 {
        return Err(ParseError::ArgumentCount);
    }
    Ok(Statement::Open {
        transaction: expression[1].get_identifier()?.clone(),
    })
}

fn build_acquire(expression: &[Expression]) -> Result<Statement, ParseError> {
    if expression.len() != 2 {
        return Err(ParseError::ArgumentCount);
    }
    Ok(Statement::Acquire {
        transaction: expression[1].get_identifier()?.clone(),
    })
}

fn build_commit(expression: &[Expression]) -> Result<Statement, ParseError> {
    if expression.len() != 2 {
        return Err(ParseError::ArgumentCount);
    }
    Ok(Statement::Commit {
        transaction: expression[1].get_identifier()?.clone(),
    })
}

fn build_close(expression: &[Expression]) -> Result<Statement, ParseError> {
    if expression.len() != 2 {
        return Err(ParseError::ArgumentCount);
    }
    Ok(Statement::Close {
        transaction: expression[1].get_identifier()?.clone(),
    })
}

fn build_select(
    expression: &[Expression],
    collections: &[Schema],
) -> Result<Statement, ParseError> {
    if expression.len() != 6 {
        return Err(ParseError::ArgumentCount);
    }
    let identifier = expression[1].get_identifier()?;
    let transaction = expression[2].get_identifier()?;
    let _lock_type = expression[3].get_identifier()?;
    let collection_expression = expression[4].get_expression()?;
    if collection_expression.len() != 2 {
        return Err(ParseError::ArgumentCount);
    }
    if collection_expression[0].get_identifier()? != "coll" {
        return Err(ParseError::UnexpectedToken);
    }
    let collection_name = collection_expression[1].get_identifier()?;
    let collection = collections
        .iter()
        .find(|s| &s.name == collection_name)
        .ok_or_else(|| ParseError::UnknownIdentifier(collection_name.clone()))?;
    let condition = build_condition(expression[5].get_expression()?, collection)?;
    Ok(Statement::Select {
        identifier: identifier.clone(),
        transaction: transaction.clone(),
        query: Query {
            collection: collection.id,
            condition,
        },
    })
}

fn build_create(
    expression: &[Expression],
    collections: &[Schema],
    reader: impl Read,
) -> Result<Statement, ParseError> {
    if expression.len() != 4 {
        return Err(ParseError::ArgumentCount);
    }
    let identifier = expression[1].get_identifier()?;
    let transaction = expression[2].get_identifier()?;
    let collection_expression = expression[3].get_expression()?;
    if collection_expression.len() != 2 {
        return Err(ParseError::ArgumentCount);
    }
    if collection_expression[0].get_identifier()? != "coll" {
        return Err(ParseError::UnexpectedToken);
    }
    let collection_name = collection_expression[1].get_identifier()?;
    let schema = collections
        .iter()
        .find(|c| &c.name == collection_name)
        .ok_or_else(|| ParseError::UnknownIdentifier(collection_name.clone()))?;
    let document = Document::from_reader(reader, schema).map_err(ParseError::TransferError)?;
    let statement = Statement::Create {
        identifier: identifier.clone(),
        transaction: transaction.clone(),
        document,
    };
    Ok(statement)
}

fn build_condition(expression: &[Expression], schema: &Schema) -> Result<Condition, ParseError> {
    if expression.is_empty() {
        return Err(ParseError::ArgumentCount);
    }
    // TODO implement true/false
    // TODO implement not equal
    match expression[0].get_operator()? {
        '=' => {
            let values = get_binary_expressions(expression, schema)?;
            Ok(Condition::Equal(values.0, values.1))
        }
        '<' => {
            let values = get_binary_expressions(expression, schema)?;
            Ok(Condition::LessThan(values.0, values.1))
        }
        '>' => {
            let values = get_binary_expressions(expression, schema)?;
            Ok(Condition::GreaterThan(values.0, values.1))
        }
        '|' => {
            let values = get_binary_conditions(expression, schema)?;
            Ok(Condition::Or(Box::new(values.0), Box::new(values.1)))
        }
        '&' => {
            let values = get_binary_conditions(expression, schema)?;
            Ok(Condition::And(Box::new(values.0), Box::new(values.1)))
        }
        '!' => {
            if expression.len() != 2 {
                return Err(ParseError::ArgumentCount);
            }
            Ok(Condition::Not(Box::new(build_condition(
                expression[1].get_expression()?,
                schema,
            )?)))
        }
        _ => unreachable!(),
    }
}

fn get_binary_expressions(
    expression: &[Expression],
    schema: &Schema,
) -> Result<(ValueExpression, ValueExpression), ParseError> {
    if expression.len() != 3 {
        return Err(ParseError::ArgumentCount);
    }
    Ok((
        build_value_expression(&expression[1], schema)?,
        build_value_expression(&expression[2], schema)?,
    ))
}

fn get_binary_conditions(
    expression: &[Expression],
    schema: &Schema,
) -> Result<(Condition, Condition), ParseError> {
    if expression.len() != 3 {
        return Err(ParseError::ArgumentCount);
    }
    return Ok((
        build_condition(expression[1].get_expression()?, schema)?,
        build_condition(expression[2].get_expression()?, schema)?,
    ));
}

fn build_value_expression(
    expression: &Expression,
    schema: &Schema,
) -> Result<ValueExpression, ParseError> {
    match expression {
        Expression::Identifier(identifier) => match identifier.as_str() {
            "true" => Ok(ValueExpression::Value(FieldValue::Bool(true))),
            "false" => Ok(ValueExpression::Value(FieldValue::Bool(false))),
            _ => Err(ParseError::UnexpectedToken),
        },
        Expression::Literal(string) => {
            Ok(ValueExpression::Value(FieldValue::String(string.clone())))
        }
        Expression::Numeric(_number) => {
            todo!("Numeric parsing")
        }
        Expression::List(expression) => {
            if expression.is_empty() {
                return Err(ParseError::ArgumentCount);
            }
            match expression[0].get_identifier()?.as_str() {
                "tf" => {
                    if expression.len() != 2 {
                        return Err(ParseError::ArgumentCount);
                    }
                    let identifier = expression[1].get_identifier()?;
                    let field = schema
                        .fields
                        .iter()
                        .find(|f| &f.name == identifier)
                        .ok_or_else(|| ParseError::UnknownIdentifier(identifier.clone()))?;
                    Ok(ValueExpression::Field(field.id))
                }
                "f" => {
                    // TODO support subexpressions in ValueExpression
                    todo!()
                }
                "s" => {
                    // TODO support subscripts in ValueExpression
                    todo!()
                }
                "num" => {
                    if expression.len() != 3 {
                        return Err(ParseError::ArgumentCount);
                    }
                    let numeric_type = expression[3].get_identifier()?;
                    let numeric_string = expression[2].get_numeric()?;
                    let field_value = match numeric_type.as_str() {
                        "Int" => FieldValue::Int(
                            numeric_string
                                .parse()
                                .map_err(|_| ParseError::NumericParseError)?,
                        ),
                        "UInt" => FieldValue::UInt(
                            numeric_string
                                .parse()
                                .map_err(|_| ParseError::NumericParseError)?,
                        ),
                        "Long" => FieldValue::Long(
                            numeric_string
                                .parse()
                                .map_err(|_| ParseError::NumericParseError)?,
                        ),
                        "ULong" => FieldValue::ULong(
                            numeric_string
                                .parse()
                                .map_err(|_| ParseError::NumericParseError)?,
                        ),
                        "Float" => FieldValue::Float(
                            numeric_string
                                .parse()
                                .map_err(|_| ParseError::NumericParseError)?,
                        ),
                        _ => return Err(ParseError::UnexpectedToken),
                    };
                    Ok(ValueExpression::Value(field_value))
                }
                _ => Err(ParseError::UnexpectedToken),
            }
        }
        _ => Err(ParseError::UnexpectedToken),
    }
}

fn build_read_all(expression: &[Expression]) -> Result<Statement, ParseError> {
    if expression.len() != 2 {
        return Err(ParseError::ArgumentCount);
    }
    let selection = expression[1].get_identifier()?;
    Ok(Statement::ReadAll {
        selection: selection.clone(),
    })
}

fn build_update_all(
    expression: &[Expression],
    selections: HashMap<String, &Selection>,
    reader: impl Read,
) -> Result<Statement, ParseError> {
    if expression.len() != 2 {
        return Err(ParseError::ArgumentCount);
    }
    let identifier = expression[1].get_identifier()?;
    let selection = selections
        .get(identifier)
        .ok_or(ParseError::UnknownIdentifier(identifier.clone()))?;
    let document =
        Document::from_reader(reader, &selection.schema).map_err(ParseError::TransferError)?;
    let statement = Statement::UpdateAll {
        selection: identifier.clone(),
        document,
    };
    Ok(statement)
}

fn build_delete(expression: &[Expression]) -> Result<Statement, ParseError> {
    if expression.len() != 2 {
        return Err(ParseError::ArgumentCount);
    }
    let identifier = expression[1].get_identifier()?;
    let statement = Statement::Delete {
        selection: identifier.clone(),
    };
    Ok(statement)
}
