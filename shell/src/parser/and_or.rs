use super::linebreak::Linebreak;
use super::pipeline::Pipeline;

use crate::parser::parseable::{Parseable, ParseError};
use crate::lexer::{Lexer, Vocabulary};

#[derive(Debug, Clone, PartialEq)]
pub enum AndOr {
    Pipeline(Pipeline),
    AndIf(Pipeline, Linebreak, Box<AndOr>),
    OrIf(Pipeline, Linebreak, Box<AndOr>),
}

impl Parseable for AndOr {
    fn parse(lexer: &mut Lexer) -> Result<Option<Self>, ParseError> where Self: Sized {
        let pipeline = Pipeline::parse(lexer)?.ok_or(ParseError::EndOfInput(format!("{}:{}", file!(), line!())))?;

        match lexer.peek() {
            Some(token) if token.vocab == Vocabulary::And => {
                lexer.next();
                let linebreak = Linebreak::parse(lexer)?.ok_or(ParseError::EndOfInput(format!("{}:{}", file!(), line!())))?;
                let next_and_or = AndOr::parse(lexer)?.ok_or(ParseError::EndOfInput(format!("{}:{}", file!(), line!())))?;
                Ok(Some(AndOr::AndIf(pipeline, linebreak, Box::new(next_and_or))))
            }
            Some(token) if token.vocab == Vocabulary::Or => {
                lexer.next();
                let linebreak = Linebreak::parse(lexer)?.ok_or(ParseError::EndOfInput(format!("{}:{}", file!(), line!())))?;
                let next_and_or = AndOr::parse(lexer)?.ok_or(ParseError::EndOfInput(format!("{}:{}", file!(), line!())))?;
                Ok(Some(AndOr::OrIf(pipeline, linebreak, Box::new(next_and_or))))
            }
            _ => Ok(Some(AndOr::Pipeline(pipeline))),
        }
    }
}
