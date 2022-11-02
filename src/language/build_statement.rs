use super::expression::Expression;

fn build_statement(expression: Vec<Expression>) {
    let keyword = get_identifier(expression.first().unwrap());
    match keyword.as_str() {
        "select" => build_select(expression),
        "read" => build_read(expression),
        _ => panic!("bad keyword"),
    }
}

fn build_select(expression: Vec<Expression>) {
    if expression.len() < 6 {
        panic!("error!");
    }
    let _selection_identifier = get_identifier(&expression[1]);
    let _transaction_identifier = get_identifier(&expression[2]);
    let _lock_type = get_identifier(&expression[3]);
    let collection_expression = get_expression(&expression[4]);
    if collection_expression.len() < 2 || get_identifier(&collection_expression[0]) != "coll" {
        panic!("error!");
    }
    let _collection_name = get_literal(&collection_expression[1]);
    let _condition = 0;
}

fn build_read(_expression: Vec<Expression>) {
    todo!()
}

fn get_identifier<'a>(expression: &'a Expression) -> &'a String {
    match expression {
        Expression::Identifier(k) => k,
        _ => panic!("unexpected token"),
    }
}

fn get_expression<'a>(expression: &'a Expression) -> &'a Vec<Expression> {
    match expression {
        Expression::List(v) => v,
        _ => panic!("unexpected token"),
    }
}

fn get_literal<'a>(expression: &'a Expression) -> &'a String {
    match expression {
        Expression::Literal(s) => s,
        _ => panic!("unexpected token"),
    }
}
