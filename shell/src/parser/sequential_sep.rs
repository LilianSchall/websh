use super::linebreak::Linebreak;
use super::newline_list::NewlineList;
use crate::parser::parseable::{Parseable, ParseError};
use crate::lexer::Lexer;
use crate::lexer::Vocabulary;

#[derive(Debug, Clone, PartialEq)]
pub enum SequentialSep {
	Semicolon(Linebreak),
	NewlineList(NewlineList),
}

impl Parseable for SequentialSep {
    fn parse(lexer: &mut Lexer) -> Result<Option<SequentialSep>, ParseError> where Self: Sized {
        match lexer.peek() {
            Some(token) if token.vocab == Vocabulary::Semicolon => {
                lexer.next();
                if let Some(linebreak) = Linebreak::parse(lexer)? {
                    return Ok(Some(SequentialSep::Semicolon(linebreak)));
                }
                return Err(ParseError::UnexpectedToken(token.representation.clone()));
            },
            _ => {
                Ok(NewlineList::parse(lexer)?.map(SequentialSep::NewlineList))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SequentialSep;
    use crate::lexer::{Lexer, Vocabulary};
    use crate::parser::linebreak::Linebreak;
    use crate::parser::newline_list::NewlineList;
    use crate::parser::parseable::Parseable;

    fn init_lexer_at_first_token(input: &str) -> Lexer<'_> {
        let mut lexer = Lexer::init(input);
        lexer.next();
        lexer
    }

    #[test]
    fn parses_semicolon_with_linebreak() {
        let mut lexer = init_lexer_at_first_token(";\n\nnext");

        let parsed = SequentialSep::parse(&mut lexer).expect("parse should not error");

        assert_eq!(
            parsed,
            Some(SequentialSep::Semicolon(Linebreak {
                newlines: Some(NewlineList { count: 2 }),
            }))
        );
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Word));
    }

    #[test]
    fn parses_newline_list_variant() {
        let mut lexer = init_lexer_at_first_token("\n\nnext");

        let parsed = SequentialSep::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, Some(SequentialSep::NewlineList(NewlineList { count: 2 })));
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Word));
    }

    #[test]
    fn returns_none_for_non_matching_input() {
        let mut lexer = init_lexer_at_first_token("echo");

        let parsed = SequentialSep::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, None);
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Word));
    }

    #[test]
    fn returns_none_on_empty_input() {
        let mut lexer = Lexer::init("");
        lexer.next();

        let parsed = SequentialSep::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, None);
        assert_eq!(lexer.peek(), None);
    }

    #[test]
    fn parses_semicolon_with_empty_linebreak() {
        let mut lexer = init_lexer_at_first_token(";echo");

        let parsed = SequentialSep::parse(&mut lexer).expect("parse should not error");

        assert_eq!(
            parsed,
            Some(SequentialSep::Semicolon(Linebreak {
                newlines: None,
            }))
        );
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Word));
    }

    #[test]
    fn parses_semicolon_at_end_of_input() {
        let mut lexer = init_lexer_at_first_token(";");

        let parsed = SequentialSep::parse(&mut lexer).expect("parse should not error");

        assert_eq!(
            parsed,
            Some(SequentialSep::Semicolon(Linebreak {
                newlines: None,
            }))
        );
        assert_eq!(lexer.peek(), None);
    }
}
