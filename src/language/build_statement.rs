use super::expression::Expression;
use super::parse_error::ParseError;
use super::statement::Statement;

fn build_statement(expression: Vec<Expression>) -> Result<Statement, ParseError> {
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

fn build_select(expression: Vec<Expression>) -> Result<Statement, ParseError> {
    if expression.len() != 6 {
        return Err(ParseError::ArgumentCount);
    }
    let identifier = expression[1].get_identifier()?;
    let transaction = expression[2].get_identifier()?;
    let lock_type = expression[3].get_identifier()?;
    let collection_expression = expression[4].get_expression()?;
    if collection_expression.len() != 2 {
        return Err(ParseError::ArgumentCount);
    }
    if collection_expression[0].get_identifier()? != "coll" {
        return Err(ParseError::UnexpectedToken);
    }
    let collection_name = collection_expression[1].get_expression()?;
    let condition = build_condition(expression[5].get_expression())?;
    Ok(Statement::Select {
        identifier: identifier.clone(),
        transaction: transaction.clone(),
        condition: condition,
    })
}

fn build_read(_expression: Vec<Expression>) -> Result<Statement, ParseError> {
    todo!()
}
