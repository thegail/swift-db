pub enum Expression {
    Node(String),
    List(Vec<Expression>),
}
