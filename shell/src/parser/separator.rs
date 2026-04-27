use super::linebreak::Linebreak;
use super::newline_list::NewlineList;
use super::separator_op::SeparatorOp;

use crate::parser::parseable::{Parseable, ParseError};
use crate::lexer::Lexer;

#[derive(Debug, Clone, PartialEq)]
pub enum Separator {
	OpLinebreak(SeparatorOp, Linebreak),
	NewlineList(NewlineList),
}

impl Parseable for Separator {
    fn parse(lexer: &mut Lexer) -> Result<Option<Separator>, ParseError> where Self: Sized {
        if let Some(op) = SeparatorOp::parse(lexer)? {
            if let Some(linebreak) = Linebreak::parse(lexer)? {
                return Ok(Some(Separator::OpLinebreak(op, linebreak)));
            }
            return Err(ParseError::EndOfInput);       
        }
        if let Some(newline_list) = NewlineList::parse(lexer)? {
            return Ok(Some(Separator::NewlineList(newline_list)));
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::Separator;
    use crate::lexer::{Lexer, Vocabulary};
    use crate::parser::linebreak::Linebreak;
    use crate::parser::newline_list::NewlineList;
    use crate::parser::parseable::Parseable;
    use crate::parser::separator_op::SeparatorOp;

    fn init_lexer_at_first_token(input: &str) -> Lexer<'_> {
        let mut lexer = Lexer::init(input);
        lexer.next();
        lexer
    }

    #[test]
    fn parses_operator_separator_with_newlines() {
        let mut lexer = init_lexer_at_first_token(";\n\necho");

        let parsed = Separator::parse(&mut lexer).expect("parse should not error");

        assert_eq!(
            parsed,
            Some(Separator::OpLinebreak(
                SeparatorOp::Semicolon,
                Linebreak {
                    newlines: Some(NewlineList { count: 2 }),
                },
            ))
        );
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Word));
    }

    #[test]
    fn parses_operator_separator_with_empty_linebreak() {
        let mut lexer = init_lexer_at_first_token("&echo");

        let parsed = Separator::parse(&mut lexer).expect("parse should not error");

        assert_eq!(
            parsed,
            Some(Separator::OpLinebreak(
                SeparatorOp::Ampersand,
                Linebreak { newlines: None },
            ))
        );
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Word));
    }

    #[test]
    fn parses_newline_list_separator_variant() {
        let mut lexer = init_lexer_at_first_token("\n\nnext");

        let parsed = Separator::parse(&mut lexer).expect("parse should not error");

        assert_eq!(
            parsed,
            Some(Separator::NewlineList(NewlineList { count: 2 }))
        );
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Word));
    }

    #[test]
    fn returns_none_for_non_separator_input() {
        let mut lexer = init_lexer_at_first_token("echo");

        let parsed = Separator::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, None);
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Word));
    }

    #[test]
    fn returns_none_on_empty_input() {
        let mut lexer = Lexer::init("");
        lexer.next();

        let parsed = Separator::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, None);
        assert_eq!(lexer.peek(), None);
    }

    #[test]
    fn does_not_error_when_operator_is_last_token() {
        let mut lexer = init_lexer_at_first_token(";");

        let parsed = Separator::parse(&mut lexer).expect("parse should not error");

        assert_eq!(
            parsed,
            Some(Separator::OpLinebreak(
                SeparatorOp::Semicolon,
                Linebreak { newlines: None },
            ))
        );
        assert_eq!(lexer.peek(), None);
    }
}
