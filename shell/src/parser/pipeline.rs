use super::pipe_sequence::PipeSequence;
use crate::lexer::{Lexer, Vocabulary};
use crate::parser::parseable::{ParseError, Parseable};

#[derive(Debug, Clone, PartialEq)]
pub struct Pipeline {
    pub bang: bool,
    pub pipe_sequence: PipeSequence,
}

impl Parseable for Pipeline {
    fn parse(lexer: &mut Lexer) -> Result<Option<Self>, ParseError>
    where
        Self: Sized,
    {
        let bang = match lexer.peek() {
            Some(token) if token.vocab == Vocabulary::Neg => {
                lexer.next();
                Ok(true)
            }
            Some(token) => Ok(false),
            None => Err(ParseError::EndOfInput(format!("{}:{}", file!(), line!()))),
        }?;

        PipeSequence::parse(lexer)?
            .map(|seq| {
                Some(Pipeline {
                    bang,
                    pipe_sequence: seq,
                })
            })
            .ok_or(ParseError::EndOfInput(format!("{}:{}", file!(), line!())))
    }
}
