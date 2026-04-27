use crate::parser::parseable::{Parseable, ParseError};
use crate::lexer::vocab::Vocabulary;
use crate::lexer::Lexer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HereEnd(pub String);

impl Parseable for HereEnd {
    fn parse(lexer: &mut Lexer) -> Result<Option<HereEnd>, ParseError> where Self: Sized {
        Ok(match lexer.peek() {
            Some(token) if token.vocab == Vocabulary::Word => {
                let here_end = token.representation.clone();
                lexer.next();
                Some(HereEnd(here_end))
            },
            _ => None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::HereEnd;
    use crate::lexer::{Lexer, Vocabulary};
    use crate::parser::parseable::Parseable;

    fn init_lexer_at_first_token(input: &str) -> Lexer<'_> {
        let mut lexer = Lexer::init(input);
        lexer.next();
        lexer
    }

    #[test]
    fn parses_word_as_here_end() {
        let mut lexer = init_lexer_at_first_token("EOF next");

        let parsed = HereEnd::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, Some(HereEnd("EOF".to_string())));
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Word));
    }

    #[test]
    fn returns_none_for_non_word_token() {
        let mut lexer = init_lexer_at_first_token(";\n");

        let parsed = HereEnd::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, None);
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Semicolon));
    }

    #[test]
    fn returns_none_on_empty_input() {
        let mut lexer = Lexer::init("");
        lexer.next();

        let parsed = HereEnd::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, None);
        assert_eq!(lexer.peek(), None);
    }
}
