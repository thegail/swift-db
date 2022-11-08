use super::parser::parse;

#[test]
fn start_parser() {
    let mut input =
        "(abc def (ghi) (jkl mno 456) \"pqr(stu() v\" (123 -953.13) \"\\\"\\\\\")".as_bytes();
    let result = parse(&mut input).expect("Parse failed");
    println!("{:?}", result);
}
