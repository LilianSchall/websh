use super::compound_command::CompoundCommand;
use super::function_definition::FunctionDefinition;
use super::redirect_list::RedirectList;
use super::simple_command::SimpleCommand;

use crate::parser::parseable::{Parseable, ParseError};
use crate::lexer::Lexer;

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
	Simple(SimpleCommand),
	Compound {
		command: CompoundCommand,
		redirects: Option<RedirectList>,
	},
	FunctionDefinition(FunctionDefinition),
}

impl Parseable for Command {
    fn parse(lexer: &mut Lexer) -> Result<Option<Self>, ParseError> where Self: Sized {
        Ok(SimpleCommand::parse(lexer)?.map(Command::Simple))
    }
}
