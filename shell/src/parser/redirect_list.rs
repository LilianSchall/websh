use super::io_redirect::IoRedirect;
use crate::parser::parseable::{Parseable, ParseError};
use crate::lexer::Lexer;
use crate::lexer::Vocabulary;

#[derive(Debug, Clone, PartialEq)]
pub struct RedirectList(pub Vec<IoRedirect>);

impl Parseable for RedirectList {
    fn parse(lexer: &mut Lexer) -> Result<Option<RedirectList>, ParseError> where Self: Sized {
        let mut redirects = Vec::new();
        loop {
            match IoRedirect::parse(lexer) {
                Ok(Some(redirect)) => redirects.push(redirect),
                Ok(None) => break,
                Err(_) => break, 
            }
        }

        if !redirects.is_empty() {
            Ok(Some(RedirectList(redirects)))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RedirectList;
    use crate::lexer::{Lexer, Vocabulary};
    use crate::parser::filename::Filename;
    use crate::parser::here_end::HereEnd;
    use crate::parser::io_file::IoFile;
    use crate::parser::io_here::IoHere;
    use crate::parser::io_redirect::{IoRedirect, IoRedirectKind};
    use crate::parser::parseable::Parseable;

    fn init_lexer_at_first_token(input: &str) -> Lexer<'_> {
        let mut lexer = Lexer::init(input);
        lexer.next();
        lexer
    }

    #[test]
    fn parses_multiple_redirects() {
        let mut lexer = init_lexer_at_first_token("< in > out ;");

        let parsed = RedirectList::parse(&mut lexer).expect("parse should not error");

        assert_eq!(
            parsed,
            Some(RedirectList(vec![
                IoRedirect {
                    io_number: None,
                    kind: IoRedirectKind::File(IoFile::Less(Filename("in".to_string()))),
                },
                IoRedirect {
                    io_number: None,
                    kind: IoRedirectKind::File(IoFile::Greater(Filename("out".to_string()))),
                },
            ]))
        );
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Semicolon));
    }

    #[test]
    fn returns_none_when_no_redirect_present() {
        let mut lexer = init_lexer_at_first_token("echo");

        let parsed = RedirectList::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, None);
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Word));
    }

    #[test]
    fn stops_parsing_when_next_token_is_not_redirect() {
        let mut lexer = init_lexer_at_first_token("< in ; > out");

        let parsed = RedirectList::parse(&mut lexer).expect("parse should not error");

        assert_eq!(
            parsed,
            Some(RedirectList(vec![IoRedirect {
                io_number: None,
                kind: IoRedirectKind::File(IoFile::Less(Filename("in".to_string()))),
            }]))
        );
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Semicolon));
    }

    #[test]
    fn parses_mixed_file_and_here_redirects() {
        let mut lexer = init_lexer_at_first_token("<< EOF > out ;");

        let parsed = RedirectList::parse(&mut lexer).expect("parse should not error");

        assert_eq!(
            parsed,
            Some(RedirectList(vec![
                IoRedirect {
                    io_number: None,
                    kind: IoRedirectKind::Here(IoHere::DLess(HereEnd("EOF".to_string()))),
                },
                IoRedirect {
                    io_number: None,
                    kind: IoRedirectKind::File(IoFile::Greater(Filename("out".to_string()))),
                },
            ]))
        );
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Semicolon));
    }

    #[test]
    fn returns_none_when_first_redirect_is_malformed() {
        let mut lexer = init_lexer_at_first_token("< ;");

        let parsed = RedirectList::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, None);
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Semicolon));
    }

    #[test]
    fn keeps_first_redirect_when_second_redirect_is_malformed() {
        let mut lexer = init_lexer_at_first_token("< in < ;");

        let parsed = RedirectList::parse(&mut lexer).expect("parse should not error");

        assert_eq!(
            parsed,
            Some(RedirectList(vec![IoRedirect {
                io_number: None,
                kind: IoRedirectKind::File(IoFile::Less(Filename("in".to_string()))),
            }]))
        );
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Semicolon));
    }

    #[test]
    fn returns_none_on_empty_input() {
        let mut lexer = Lexer::init("");
        lexer.next();

        let parsed = RedirectList::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, None);
        assert_eq!(lexer.peek(), None);
    }
}
