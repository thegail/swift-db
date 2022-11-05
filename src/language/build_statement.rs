use super::expression::Expression;
use super::parse_error::ParseError;

fn build_statement(expression: Vec<Expression>) -> Result<(), ParseError> {
    let keyword = expression.first().unwrap().get_identifier();
    match keyword.as_str() {
        "select" => build_select(expression),
        "read" => build_read(expression),
        _ => panic!("bad keyword"),
    }
}

fn build_select(expression: Vec<Expression>) -> Result<(), ParseError> {
    if expression.len() != 6 {
        return Err(ParseError::ArgumentCount);
    }
    expression[1].get_identifier();
    expression[2].get_identifier();
    expression[3].get_identifier();
    expression[4].get_expression();
    if collection_expression.len() < 2 || collection_expression[0].get_identifier() != "coll" {
        panic!("error!");
    }
    let _collection_name = collection_expression[1].get_literal();
    let _condition = 0;
    Ok(())
}

fn build_read(_expression: Vec<Expression>) -> Result<(), ParseError> {
    todo!()
}
