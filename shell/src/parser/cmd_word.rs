use crate::parser::parseable::{Parseable, ParseError};
use crate::lexer::{Lexer, Vocabulary};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmdWord(pub String);

impl Parseable for CmdWord {
    fn parse(lexer: &mut Lexer) -> Result<Option<Self>, ParseError> where Self: Sized {
        match lexer.peek() {
            Some(token) if token.vocab == Vocabulary::Word => {
                lexer.next();
                Ok(Some(CmdWord(token.representation)))
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
    use super::CmdWord;
    use crate::lexer::{Lexer, Vocabulary};
    use crate::parser::parseable::{Parseable, ParseError};

    fn init_lexer_at_first_token(input: &str) -> Lexer<'_> {
        let mut lexer = Lexer::init(input);
        lexer.next();
        lexer
    }

    #[test]
    fn parses_word_token_as_cmd_word() {
        let mut lexer = init_lexer_at_first_token("echo next");

        let parsed = CmdWord::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, Some(CmdWord("echo".to_string())));
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Word));
    }

    #[test]
    fn returns_none_for_non_word_token() {
        let mut lexer = init_lexer_at_first_token(";\n");

        let parsed = CmdWord::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, None);
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Semicolon));
    }

    #[test]
    fn returns_end_of_input_error_on_empty_input() {
        let mut lexer = Lexer::init("");
        lexer.next();

        let parsed = CmdWord::parse(&mut lexer);

        assert!(matches!(parsed, Err(ParseError::EndOfInput)));
    }

    #[test]
    fn consumes_only_first_word_token() {
        let mut lexer = init_lexer_at_first_token("echo hello");

        let parsed = CmdWord::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, Some(CmdWord("echo".to_string())));
        assert_eq!(lexer.peek().map(|token| token.representation), Some("hello".to_string()));
    }

    #[test]
    fn returns_none_for_reserved_word_token() {
        let mut lexer = init_lexer_at_first_token("if cmd");

        let parsed = CmdWord::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, None);
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::If));
    }
}
