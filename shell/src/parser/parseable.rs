use crate::lexer::Lexer;

#[derive(Debug, Clone)]
pub enum ParseError {
    UnexpectedToken(String),
    InvalidIoNumber(String),
    EndOfInput,
}

pub trait Parseable {
    fn parse(lexer: &mut Lexer) -> Result<Option<Self>, ParseError> where Self: Sized;
}
