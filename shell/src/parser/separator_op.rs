use crate::parser::parseable::{Parseable, ParseError};
use crate::lexer::vocab::Vocabulary;
use crate::lexer::Lexer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeparatorOp {
	Ampersand,
	Semicolon,
}

impl Parseable for SeparatorOp {
    fn parse(lexer: &mut Lexer) -> Result<Option<SeparatorOp>, ParseError> where Self: Sized {
        Ok(match lexer.peek() {
            Some(token) if token.vocab == Vocabulary::Ampersand => {
                lexer.next();
                Some(SeparatorOp::Ampersand)
            },
            Some(token) if token.vocab == Vocabulary::Semicolon => {
                lexer.next();
                Some(SeparatorOp::Semicolon)
            },
            _ => None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::SeparatorOp;
    use crate::lexer::{Lexer, Vocabulary};
    use crate::parser::parseable::Parseable;

    fn init_lexer_at_first_token(input: &str) -> Lexer<'_> {
        let mut lexer = Lexer::init(input);
        lexer.next();
        lexer
    }

    #[test]
    fn parses_ampersand_operator() {
        let mut lexer = init_lexer_at_first_token("& echo");

        let parsed = SeparatorOp::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, Some(SeparatorOp::Ampersand));
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Word));
    }

    #[test]
    fn parses_semicolon_operator() {
        let mut lexer = init_lexer_at_first_token(";\n");

        let parsed = SeparatorOp::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, Some(SeparatorOp::Semicolon));
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Newline));
    }

    #[test]
    fn returns_none_for_non_separator_token() {
        let mut lexer = init_lexer_at_first_token("echo");

        let parsed = SeparatorOp::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, None);
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Word));
    }

    #[test]
    fn returns_none_on_empty_input() {
        let mut lexer = Lexer::init("");
        lexer.next();

        let parsed = SeparatorOp::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, None);
        assert_eq!(lexer.peek(), None);
    }
}
