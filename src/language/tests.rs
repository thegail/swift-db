use super::parser::parse;

#[test]
fn start_parser() {
    let input =
        "(abc def (ghi) (jkl mno 456) \"pqr(stu() v\" (123 -953.13) \"\\\"\\\\\")".as_bytes();
    let result = parse(input).expect("Parse failed");
    println!("{:?}", result);
}
