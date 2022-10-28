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
    current_type: Option<CurrentType>,
    is_escaped: bool,
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
            current_type: None,
            is_escaped: false,
        }
    }

    fn parse_input(mut self) -> Result<Vec<Expression>, ParseError> {
        for byte in self.input.bytes() {
            let byte = byte.map_err(|e| ParseError::ReadError(e))?;
            if let Some(CurrentType::Literal) = self.current_type {
                if self.is_escaped {
                    match byte {
                        b'"' => self.current.push(byte as char),
                        b'\\' => self.current.push(byte as char),
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
                b' ' => match self.current_type {
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
                },
                b'"' => match self.current_type {
                    None => self.current_type = Some(CurrentType::Literal),
                    _ => return Err(ParseError::UnexpectedCharacter(byte)),
                },
                c => return Err(ParseError::UnexpectedCharacter(c)),
            }
        }
        Ok(self.output.pop().unwrap())
    }

    pub fn parse(input: I) -> Result<Vec<Expression>, ParseError> {
        let parser = Self::new(input);
        parser.parse_input()
    }
}

enum CurrentType {
    Identifier,
    Numeric,
    Literal,
}
