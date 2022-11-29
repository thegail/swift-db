use super::expression::Expression;
use crate::language::ParseError;
use std::io::Read;

struct Parser {
    position: usize,
    output: Vec<Vec<Expression>>,
    current: String,
    current_type: Option<CurrentType>,
    is_escaped: bool,
}

impl Parser {
    fn new() -> Self {
        Self {
            position: 0,
            output: Vec::new(),
            current: String::new(),
            current_type: None,
            is_escaped: false,
        }
    }

    fn parse_input(mut self, input: impl Read) -> Result<Vec<Expression>, ParseError> {
        for byte in input.bytes() {
            let byte = byte.map_err(ParseError::ReadError)?;
            if self.output.is_empty() {
                match byte {
                    b' ' | b'\n' => continue,
                    b'(' => (),
                    _ => return Err(ParseError::UnexpectedToken),
                }
            }
            if let Some(CurrentType::Literal) = self.current_type {
                if self.is_escaped {
                    match byte {
                        b'"' | b'\\' => {
                            self.current.push(byte as char);
                            self.is_escaped = false;
                        }
                        _ => {
                            return Err(ParseError::UnexpectedCharacter {
                                position: self.position,
                                value: byte,
                            })
                        }
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
                    } else if self.output.is_empty() {
                        return Err(ParseError::UnexpectedCharacter {
                            position: self.position,
                            value: byte,
                        });
                    }
                    let list = self.output.pop().unwrap();
                    let higher = self.output.last_mut();
                    if higher.is_none() {
                        break;
                    }
                    higher.unwrap().push(Expression::List(list));
                }
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                    match self.current_type {
                        None => self.current_type = Some(CurrentType::Identifier),
                        Some(CurrentType::Identifier) => (),
                        _ => {
                            return Err(ParseError::UnexpectedCharacter {
                                position: self.position,
                                value: byte,
                            })
                        }
                    }
                    self.current.push(byte as char);
                }
                b'0'..=b'9' | b'-' | b'.' => {
                    match self.current_type {
                        None => self.current_type = Some(CurrentType::Numeric),
                        Some(CurrentType::Numeric) => (),
                        _ => {
                            return Err(ParseError::UnexpectedCharacter {
                                position: self.position,
                                value: byte,
                            })
                        }
                    }
                    self.current.push(byte as char);
                }
                b'=' | b'<' | b'>' | b'|' | b'&' | b'!' => match self.current_type {
                    None => self
                        .output
                        .last_mut()
                        .unwrap()
                        .push(Expression::Operator(byte as char)),
                    _ => {
                        return Err(ParseError::UnexpectedCharacter {
                            position: self.position,
                            value: byte,
                        })
                    }
                },
                b' ' | b'\n' => self.end_token(byte)?,
                b'"' => match self.current_type {
                    None => self.current_type = Some(CurrentType::Literal),
                    _ => {
                        return Err(ParseError::UnexpectedCharacter {
                            position: self.position,
                            value: byte,
                        })
                    }
                },
                _ => {
                    return Err(ParseError::UnexpectedCharacter {
                        position: self.position,
                        value: byte,
                    })
                }
            }
        }
        if self.output.is_empty() {
            Err(ParseError::UnexpectedEndOfInput)
        } else {
            Ok(self.output.pop().unwrap())
        }
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
            _ => {
                return Err(ParseError::UnexpectedCharacter {
                    position: self.position,
                    value: byte,
                })
            }
        }
        Ok(())
    }
}

enum CurrentType {
    Identifier,
    Numeric,
    Literal,
}

/// Read one s-expression from an input stream.
///
/// Returns a [`Vec<Expression>`] or a [`ParseError`]. The parsed
/// expression should be passed to [`build_statement`] to generate
/// an executable [`Statement`].
///
/// [`build_statement`]: crate::language::build_statement
/// [`Statement`]: crate::language::Statement
pub fn parse(input: impl Read) -> Result<Vec<Expression>, ParseError> {
    let parser = Parser::new();
    parser.parse_input(input)
}
