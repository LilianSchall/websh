use super::list::List;
use super::separator::Separator;

use crate::lexer::Lexer;
use crate::parser::parseable::{Parseable, ParseError};

#[derive(Debug, Clone, PartialEq)]
pub struct CompleteCommand {
	pub list: List,
	pub separator: Option<Separator>,
}

impl Parseable for crate::parser::complete_command::CompleteCommand {
    fn parse(lexer: &mut Lexer) -> Result<Option<Self>, ParseError> where Self: Sized {
        let list = List::parse(lexer)?.unwrap();
        // Separator is optional, so we ignore parsing errors
        let separator = Separator::parse(lexer).ok().unwrap_or(None); 
        Ok(Some(CompleteCommand { list, separator }))
    }
}
