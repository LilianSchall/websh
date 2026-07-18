use super::command::Command;
use super::linebreak::Linebreak;

use crate::lexer::{Lexer, Vocabulary};
use crate::parser::parseable::{ParseError, Parseable};

#[derive(Debug, Clone, PartialEq)]
pub enum PipeSequence {
    Command(Command),
    Pipe(Command, Linebreak, Box<PipeSequence>),
}

impl Parseable for PipeSequence {
    fn parse(lexer: &mut Lexer) -> Result<Option<Self>, ParseError>
    where
        Self: Sized,
    {
        let command = Command::parse(lexer)?.ok_or(ParseError::EndOfInput(format!(
            "{}:{}",
            file!(),
            line!()
        )))?;

        match lexer.peek() {
            Some(token) if token.vocab == Vocabulary::RPipe => {
                lexer.next();
                let linebreak = Linebreak::parse(lexer)?.ok_or(ParseError::EndOfInput(format!(
                    "{}:{}",
                    file!(),
                    line!()
                )))?;
                let next_sequence = PipeSequence::parse(lexer)?
                    .ok_or(ParseError::EndOfInput(format!("{}:{}", file!(), line!())))?;
                Ok(Some(PipeSequence::Pipe(
                    command,
                    linebreak,
                    Box::new(next_sequence),
                )))
            }
            _ => Ok(Some(PipeSequence::Command(command))),
        }
    }
}
