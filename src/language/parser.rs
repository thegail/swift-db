use super::expression::Expression;
use super::parse_error::ParseError;
use std::io::Read;

struct Parser<I>
where
    I: Read,
{
    input: I,
    pos: usize,
    is_free: bool,
    output: Vec<Vec<Expression>>,
    current: String,
}

impl<I> Parser<I>
where
    I: Read,
{
    fn new(input: I) -> Self {
        Self {
            input,
            pos: 0,
            is_free: true,
            output: Vec::new(),
            current: String::new(),
        }
    }

    fn parse(mut self) -> Result<Vec<Expression>, ParseError> {
        for byte in self.input.bytes() {
            let byte = byte.map_err(|e| ParseError::ReadError(e))?;
            match byte {
                b'(' => self.output.push(Vec::new()),
                b')' => {
                    let list = self.output.pop().unwrap();
                    let mut higher = self.output.last_mut();
                    if let None = higher {
                        return Ok(list);
                    }
                    let mut higher = higher.unwrap();
                    higher.push(Expression::List(list));
                }
                b' ' => (),
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => (),
                b'0'..=b'9' => (),
                b'\\' => (),
                b'"' => (),
                c => return Err(ParseError::UnexpectedCharacter(c)),
            }
        }
        Ok(vec![])
    }
}
