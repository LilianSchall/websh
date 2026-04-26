use crate::lexer::Lexer;

pub trait Parseable {
    fn parse(&self, lexer: &mut Lexer) -> Option<Self> where Self: Sized;
}
