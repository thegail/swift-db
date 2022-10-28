use super::expression::Expression;
use super::parse_error::ParseError;
use std::io::Read;

struct Parser {
    pos: usize,
    is_free: bool,
    output: Vec<Vec<Expression>>,
    current: String,
    current_type: Option<CurrentType>,
    is_escaped: bool,
}

impl Parser {
    fn new() -> Self {
        Self {
            pos: 0,
            is_free: true,
            output: Vec::new(),
            current: String::new(),
            current_type: None,
            is_escaped: false,
        }
    }

    fn parse_input(mut self, input: impl Read) -> Result<Vec<Expression>, ParseError> {
        for byte in input.bytes() {
            let byte = byte.map_err(|e| ParseError::ReadError(e))?;
            if let Some(CurrentType::Literal) = self.current_type {
                if self.is_escaped {
                    match byte {
                        b'"' | b'\\' => {
                            self.current.push(byte as char);
                            self.is_escaped = false;
                        }
                        _ => return Err(ParseError::UnexpectedCharacter(byte)),
                    }
                } else if byte == b'\\' {
                    self.is_escaped = true;
                } else if byte == b'"' {
                    self.output
                        .last_mut()
                        .unwrap()
                        .push(Expression::Literal(self.current.clone()));
                    self.current.clear();
                    self.current_type = None;
                } else {
                    self.current.push(byte as char);
                }
                continue;
            }
            match byte {
                b'(' => self.output.push(Vec::new()),
                b')' => {
                    self.end_token(byte)?;
                    if self.output.len() == 1 {
                        break;
                    } else if self.output.len() == 0 {
                        return Err(ParseError::UnexpectedCharacter(b')'));
                    }
                    let list = self.output.pop().unwrap();
                    let higher = self.output.last_mut();
                    if let None = higher {
                        break;
                    }
                    higher.unwrap().push(Expression::List(list));
                }
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                    match self.current_type {
                        None => self.current_type = Some(CurrentType::Identifier),
                        Some(CurrentType::Identifier) => (),
                        _ => return Err(ParseError::UnexpectedCharacter(byte)),
                    }
                    self.current.push(byte as char);
                }
                b'0'..=b'9' | b'-' | b'.' => {
                    match self.current_type {
                        None => self.current_type = Some(CurrentType::Numeric),
                        Some(CurrentType::Numeric) => (),
                        _ => return Err(ParseError::UnexpectedCharacter(byte)),
                    }
                    self.current.push(byte as char);
                }
                b' ' => self.end_token(byte)?,
                b'"' => match self.current_type {
                    None => self.current_type = Some(CurrentType::Literal),
                    _ => return Err(ParseError::UnexpectedCharacter(byte)),
                },
                c => return Err(ParseError::UnexpectedCharacter(c)),
            }
        }
        Ok(self.output.pop().unwrap())
    }

    fn end_token(&mut self, byte: u8) -> Result<(), ParseError> {
        match self.current_type {
            Some(CurrentType::Identifier) => {
                self.output
                    .last_mut()
                    .unwrap()
                    .push(Expression::Identifier(self.current.clone()));
                self.current.clear();
                self.current_type = None;
            }
            Some(CurrentType::Numeric) => {
                self.output
                    .last_mut()
                    .unwrap()
                    .push(Expression::Numeric(self.current.clone()));
                self.current.clear();
                self.current_type = None;
            }
            None => (),
            _ => return Err(ParseError::UnexpectedCharacter(byte)),
        }
        Ok(())
    }
}

pub fn parse(input: impl Read) -> Result<Vec<Expression>, ParseError> {
    let parser = Parser::new();
    parser.parse_input(input)
}

enum CurrentType {
    Identifier,
    Numeric,
    Literal,
}
