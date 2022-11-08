use super::expression::Expression;
use super::parse_error::ParseError;
use super::statement::Statement;
use crate::backend::{Condition, Expression as ValueExpression, Query};
use crate::schema::{FieldValue, Schema};

pub fn build_statement(expression: &[Expression]) -> Result<Statement, ParseError> {
    let keyword = expression
        .first()
        .ok_or(ParseError::ArgumentCount)?
        .get_identifier()?;
    match keyword.as_str() {
        "open" => build_open(expression),
        "acquire" => build_acquire(expression),
        "commit" => build_commit(expression),
        "close" => build_close(expression),
        "select" => build_select(expression),
        "readall" => build_read_all(expression),
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

fn build_select(expression: &[Expression]) -> Result<Statement, ParseError> {
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
    // TODO get collection id
    let _collection_name = collection_expression[1].get_expression()?;
    let collection = 0;
    let condition = build_condition(expression[5].get_expression()?)?;
    Ok(Statement::Select {
        identifier: identifier.clone(),
        transaction: transaction.clone(),
        query: Query {
            collection,
            condition,
        },
    })
}

fn build_condition(expression: &[Expression]) -> Result<Condition, ParseError> {
    if expression.is_empty() {
        return Err(ParseError::ArgumentCount);
    }
    // TODO implement true/false
    // TODO implement not equal
    match expression[0].get_operator()? {
        '=' => {
            let values = get_binary_expressions(expression)?;
            Ok(Condition::Equal(values.0, values.1))
        }
        '<' => {
            let values = get_binary_expressions(expression)?;
            Ok(Condition::LessThan(values.0, values.1))
        }
        '>' => {
            let values = get_binary_expressions(expression)?;
            Ok(Condition::GreaterThan(values.0, values.1))
        }
        '|' => {
            let values = get_binary_conditions(expression)?;
            Ok(Condition::Or(Box::new(values.0), Box::new(values.1)))
        }
        '&' => {
            let values = get_binary_conditions(expression)?;
            Ok(Condition::And(Box::new(values.0), Box::new(values.1)))
        }
        '!' => {
            if expression.len() != 2 {
                return Err(ParseError::ArgumentCount);
            }
            Ok(Condition::Not(Box::new(build_condition(
                expression[1].get_expression()?,
            )?)))
        }
        _ => unreachable!(),
    }
}

fn get_binary_expressions(
    expression: &[Expression],
) -> Result<(ValueExpression, ValueExpression), ParseError> {
    if expression.len() != 3 {
        return Err(ParseError::ArgumentCount);
    }
    Ok((
        build_value_expression(&expression[1])?,
        build_value_expression(&expression[2])?,
    ))
}

fn get_binary_conditions(expression: &[Expression]) -> Result<(Condition, Condition), ParseError> {
    if expression.len() != 3 {
        return Err(ParseError::ArgumentCount);
    }
    return Ok((
        build_condition(expression[1].get_expression()?)?,
        build_condition(expression[2].get_expression()?)?,
    ));
}

fn build_value_expression(expression: &Expression) -> Result<ValueExpression, ParseError> {
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
                    // TODO get schema here
                    let schema = Schema {
                        id: 0,
                        name: "abc".to_string(),
                        fields: vec![],
                    };
                    let field = schema
                        .fields
                        .iter()
                        .find(|f| f.name == identifier.as_str())
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
