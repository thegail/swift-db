use super::parser::parse;
use std::fs::File;
use std::io::Write;

#[test]
fn start_parser() {
    {
        let input = "(abc def (ghi) (jkl mno 456) \"pqr(stu() v\" (123 -953.13) \"\\\"\\\\\")";
        let mut file = File::create("/tmp/rust_parser_tmp.txt").expect("Failed write open");
        file.write_all(input.as_bytes()).expect("Failed write");
    }
    let file = File::open("/tmp/rust_parser_tmp.txt").expect("Failed read open");
    let result = parse(file).expect("Parse failed");
    println!("{:?}", result);
}
