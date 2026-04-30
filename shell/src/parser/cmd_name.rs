use crate::parser::parseable::{Parseable, ParseError};
use crate::lexer::{Lexer, Vocabulary};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmdName(pub String);

impl Parseable for CmdName {
    fn parse(lexer: &mut Lexer) -> Result<Option<Self>, ParseError> where Self: Sized {
        match lexer.peek() {
            Some(token) if token.vocab == Vocabulary::Word => {
                lexer.next();
                Ok(Some(CmdName(token.representation)))
            },
            Some(_) => {
                Ok(None)
            }
            None => Err(ParseError::EndOfInput),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CmdName;
    use crate::lexer::{Lexer, Vocabulary};
    use crate::parser::parseable::{Parseable, ParseError};

    fn init_lexer_at_first_token(input: &str) -> Lexer<'_> {
        let mut lexer = Lexer::init(input);
        lexer.next();
        lexer
    }

    #[test]
    fn parses_word_token_as_cmd_name() {
        let mut lexer = init_lexer_at_first_token("echo next");

        let parsed = CmdName::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, Some(CmdName("echo".to_string())));
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Word));
    }

    #[test]
    fn returns_none_for_non_word_token() {
        let mut lexer = init_lexer_at_first_token("&& x");

        let parsed = CmdName::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, None);
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::And));
    }

    #[test]
    fn returns_end_of_input_error_on_empty_input() {
        let mut lexer = Lexer::init("");
        lexer.next();

        let parsed = CmdName::parse(&mut lexer);

        assert!(matches!(parsed, Err(ParseError::EndOfInput)));
    }

    #[test]
    fn consumes_only_first_word_token() {
        let mut lexer = init_lexer_at_first_token("printf value");

        let parsed = CmdName::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, Some(CmdName("printf".to_string())));
        assert_eq!(lexer.peek().map(|token| token.representation), Some("value".to_string()));
    }

    #[test]
    fn returns_none_for_reserved_word_token() {
        let mut lexer = init_lexer_at_first_token("while x");

        let parsed = CmdName::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, None);
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::While));
    }
}
