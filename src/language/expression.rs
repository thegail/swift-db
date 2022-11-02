#[derive(Debug)]
pub enum Expression {
    Identifier(String),
    Numeric(String),
    Literal(String),
    List(Vec<Expression>),
}

impl Expression {
    pub fn get_identifier(&self) -> &String {
        match self {
            Expression::Identifier(k) => k,
            _ => panic!("unexpected token"),
        }
    }

    pub fn get_expression(&self) -> &Vec<Expression> {
        match self {
            Expression::List(v) => v,
            _ => panic!("unexpected token"),
        }
    }

    pub fn get_literal(&self) -> &String {
        match self {
            Expression::Literal(s) => s,
            _ => panic!("unexpected token"),
        }
    }
}
