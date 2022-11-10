//! The `language` parser is responsible for transforming
//! a byte stream into executable [`Statement`]s, and
//! serializing [`Response`]s.
//!
//! The [`parse`] function yields parses incoming s-expressions,
//! yeilding a [`Vec<Expression>`], which then is assembled into
//! an executable [`Statement`] by the [`build_statement`] function.
//! Additionally, the [`frontend`][crate::frontend]'s [`Response`]
//! statements are serialized into s-expressions with
//! [`Response::serialize`] and returned to the client.
mod build_statement;
mod expression;
mod parse_error;
mod parser;
mod response;
mod statement;
#[cfg(test)]
mod tests;

pub use build_statement::build_statement;
pub use parse_error::ParseError;
pub use parser::parse;
pub use response::Response;
pub use statement::Statement;
