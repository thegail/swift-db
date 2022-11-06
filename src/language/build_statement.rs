use super::expression::Expression;
use super::parse_error::ParseError;
use super::statement::Statement;
use crate::backend::{Condition, Expression as ValueExpression, Query};

fn build_statement(expression: &Vec<Expression>) -> Result<Statement, ParseError> {
    let keyword = expression
        .first()
        .ok_or(ParseError::ArgumentCount)?
        .get_identifier()?;
    match keyword.as_str() {
        "select" => build_select(expression),
        "read" => build_read(expression),
        _ => panic!("bad keyword"),
    }
}

fn build_select(expression: &Vec<Expression>) -> Result<Statement, ParseError> {
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

fn build_condition(expression: &Vec<Expression>) -> Result<Condition, ParseError> {
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
    expression: &Vec<Expression>,
) -> Result<(ValueExpression, ValueExpression), ParseError> {
    if expression.len() != 3 {
        return Err(ParseError::ArgumentCount);
    }
    return Ok((
        build_value_expression(&expression[1])?,
        build_value_expression(&expression[2])?,
    ));
}

fn get_binary_conditions(
    expression: &Vec<Expression>,
) -> Result<(Condition, Condition), ParseError> {
    if expression.len() != 3 {
        return Err(ParseError::ArgumentCount);
    }
    return Ok((
        build_condition(expression[1].get_expression()?)?,
        build_condition(expression[2].get_expression()?)?,
    ));
}

fn build_value_expression(expression: &Expression) -> Result<ValueExpression, ParseError> {
    todo!()
}

fn build_read(_expression: &Vec<Expression>) -> Result<Statement, ParseError> {
    todo!()
}
