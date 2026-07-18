use crate::lexer::{Lexer, Token, Vocabulary};
use crate::parser::parseable::Parseable;

use crate::parser::complete_command;

pub fn parse(lexer: &mut Lexer) -> Option<complete_command::CompleteCommand> {
    complete_command::CompleteCommand::parse(lexer)
        .ok()
        .unwrap_or(None)
}
