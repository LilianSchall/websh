use crate::parser::parseable::{Parseable, ParseError};
use crate::lexer::vocab::Vocabulary;
use crate::lexer::Lexer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Filename(pub String);

impl Parseable for Filename {
    fn parse(lexer: &mut Lexer) -> Result<Option<Filename>, ParseError> where Self: Sized {
        match lexer.peek() {
            Some(token) if token.vocab.is_word() => {
                let filename = token.representation.clone();
                lexer.next();
                Ok(Some(Filename(filename)))
            }
            _ => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Filename;
    use crate::lexer::{Lexer, Vocabulary};
    use crate::parser::parseable::Parseable;

    fn init_lexer_at_first_token(input: &str) -> Lexer<'_> {
        let mut lexer = Lexer::init(input);
        lexer.next();
        lexer
    }

    #[test]
    fn parses_word_as_filename() {
        let mut lexer = init_lexer_at_first_token("file.txt rest");

        let parsed = Filename::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, Some(Filename("file.txt".to_string())));
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Word));
    }

    #[test]
    fn returns_none_for_non_word_token() {
        let mut lexer = init_lexer_at_first_token("&& x");

        let parsed = Filename::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, None);
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::And));
    }

    #[test]
    fn returns_none_on_empty_input() {
        let mut lexer = Lexer::init("");
        lexer.next();

        let parsed = Filename::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, None);
        assert_eq!(lexer.peek(), None);
    }
}
