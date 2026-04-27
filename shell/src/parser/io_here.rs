use super::here_end::HereEnd;
use crate::parser::parseable::{Parseable, ParseError};
use crate::lexer::Lexer;
use crate::lexer::Vocabulary;

#[derive(Debug, Clone, PartialEq)]
pub enum IoHere {
	DLess(HereEnd),
	DLessDash(HereEnd),
}

impl Parseable for IoHere {
    fn parse(lexer: &mut Lexer) -> Result<Option<IoHere>, ParseError> where Self: Sized {
        Ok(match lexer.peek() {
            Some(token) => match token.vocab {
                Vocabulary::InfInf => {
                    lexer.next();
                    HereEnd::parse(lexer)?.map(IoHere::DLess)
                },
                Vocabulary::InfInfMin => {
                    lexer.next();
                    HereEnd::parse(lexer)?.map(IoHere::DLessDash)
                },
                _ => None,
            },
            None => None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::IoHere;
    use crate::lexer::{Lexer, Vocabulary};
    use crate::parser::here_end::HereEnd;
    use crate::parser::parseable::Parseable;

    fn init_lexer_at_first_token(input: &str) -> Lexer<'_> {
        let mut lexer = Lexer::init(input);
        lexer.next();
        lexer
    }

    #[test]
    fn parses_dless_here_document_redirect() {
        let mut lexer = init_lexer_at_first_token("<< EOF rest");

        let parsed = IoHere::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, Some(IoHere::DLess(HereEnd("EOF".to_string()))));
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Word));
    }

    #[test]
    fn parses_dlessdash_here_document_redirect() {
        let mut lexer = init_lexer_at_first_token("<<- EOF rest");

        let parsed = IoHere::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, Some(IoHere::DLessDash(HereEnd("EOF".to_string()))));
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Word));
    }

    #[test]
    fn returns_none_for_non_io_here_start_token() {
        let mut lexer = init_lexer_at_first_token("echo");

        let parsed = IoHere::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, None);
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Word));
    }

    #[test]
    fn returns_none_when_here_end_is_missing_after_dless() {
        let mut lexer = init_lexer_at_first_token("<< ;");

        let parsed = IoHere::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, None);
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Semicolon));
    }

    #[test]
    fn returns_none_when_here_end_is_missing_after_dlessdash() {
        let mut lexer = init_lexer_at_first_token("<<- ;");

        let parsed = IoHere::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, None);
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Semicolon));
    }

    #[test]
    fn returns_none_on_empty_input() {
        let mut lexer = Lexer::init("");
        lexer.next();

        let parsed = IoHere::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, None);
        assert_eq!(lexer.peek(), None);
    }
}
