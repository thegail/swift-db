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

    fn parse(mut self) -> Result<(), ParseError> {
        for byte in self.input.bytes() {
            let byte = byte.map_err(|e| ParseError::ReadError(e))?;
            match byte {
                b'(' => self.output.push(Vec::new()),
                b')' => {
                    if self.output.len() == 1 {
                        return Ok(());
                    } else if self.output.len() == 0 {
                        return Err(ParseError::UnexpectedCharacter(b')'));
                    }
                    let list = self.output.pop().unwrap();
                    let higher = self.output.last_mut();
                    if let None = higher {
                        return Ok(());
                    }
                    higher.unwrap().push(Expression::List(list));
                }
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => (),
                b'0'..=b'9' => (),
                b' ' => (),
                b'"' => (),
                b'\\' => (),
                c => return Err(ParseError::UnexpectedCharacter(c)),
            }
        }
        Ok(())
    }
}
