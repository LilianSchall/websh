use crate::lexer::Lexer;
use crate::parser::parseable::{Parseable, ParseError};

use super::and_or::AndOr;
use super::separator_op::SeparatorOp;

#[derive(Debug, Clone, PartialEq)]
pub enum List {
    AndOr(AndOr),
    SeparatorOp(Box<List>, SeparatorOp, AndOr),
}

impl Parseable for List {
    fn parse(lexer: &mut Lexer) -> Result<Option<Self>, ParseError> {
        let and_or = AndOr::parse(lexer)?.ok_or(ParseError::EndOfInput(format!("{}:{}", file!(), line!())))?;

        let separator_op = SeparatorOp::parse(lexer).ok().unwrap_or(None);

        if let Some(sep_op) = separator_op {
            let next_list = List::parse(lexer)?.ok_or(ParseError::EndOfInput(format!("{}:{}", file!(), line!())))?;
            return Ok(Some(List::SeparatorOp(Box::new(next_list), sep_op, and_or)));
        }
        else {
            return Ok(Some(List::AndOr(and_or)));
        }

    }
}
