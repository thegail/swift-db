use super::parse_error::ParseError;

#[derive(Debug)]
pub enum Expression {
    Identifier(String),
    Numeric(String),
    Literal(String),
    Operator(char),
    List(Vec<Expression>),
}

impl Expression {
    pub fn get_identifier(&self) -> Result<&String, ParseError> {
        match self {
            Expression::Identifier(s) => Ok(s),
            _ => Err(ParseError::UnexpectedToken),
        }
    }

    pub fn get_expression(&self) -> Result<&Vec<Expression>, ParseError> {
        match self {
            Expression::List(v) => Ok(v),
            _ => Err(ParseError::UnexpectedToken),
        }
    }

    pub fn get_literal(&self) -> Result<&String, ParseError> {
        match self {
            Expression::Literal(s) => Ok(s),
            _ => Err(ParseError::UnexpectedToken),
        }
    }

    pub fn get_operator(&self) -> Result<char, ParseError> {
        match self {
            Expression::Operator(c) => Ok(*c),
            _ => Err(ParseError::UnexpectedToken),
        }
    }
}
