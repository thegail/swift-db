use super::expression::Expression;

fn build_statement(expression: Vec<Expression>) {
    let keyword = expression.first().unwrap().get_identifier();
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
    let _selection_identifier = expression[1].get_identifier();
    let _transaction_identifier = expression[2].get_identifier();
    let _lock_type = expression[3].get_identifier();
    let collection_expression = expression[4].get_expression();
    if collection_expression.len() < 2 || collection_expression[0].get_identifier() != "coll" {
        panic!("error!");
    }
    let _collection_name = collection_expression[1].get_literal();
    let _condition = 0;
}

fn build_read(_expression: Vec<Expression>) {
    todo!()
}
