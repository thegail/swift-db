#[derive(Debug)]
pub enum Expression {
    Identifier(String),
    Numeric(String),
    Literal(String),
    List(Vec<Expression>),
}

impl Expression {
    pub fn get_identifier<'a>(&'a self) -> &'a String {
        match self {
            Expression::Identifier(k) => k,
            _ => panic!("unexpected token"),
        }
    }

    pub fn get_expression<'a>(&'a self) -> &'a Vec<Expression> {
        match self {
            Expression::List(v) => v,
            _ => panic!("unexpected token"),
        }
    }

    pub fn get_literal<'a>(&'a self) -> &'a String {
        match self {
            Expression::Literal(s) => s,
            _ => panic!("unexpected token"),
        }
    }
}
