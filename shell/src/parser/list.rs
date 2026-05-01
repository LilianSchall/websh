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
        todo!()
    }
}
