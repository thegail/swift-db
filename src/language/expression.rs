pub enum Expression {
    Identifier(String),
    Numeric(String),
    Literal(String),
    List(Vec<Expression>),
}
