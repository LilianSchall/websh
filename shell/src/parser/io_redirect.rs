use super::io_file::IoFile;
use super::io_here::IoHere;
use crate::parser::parseable::{Parseable, ParseError};
use crate::lexer::Lexer;
use crate::lexer::Vocabulary;

#[derive(Debug, Clone, PartialEq)]
pub enum IoRedirectKind {
	File(IoFile),
	Here(IoHere),
}

#[derive(Debug, Clone, PartialEq)]
pub struct IoRedirect {
	pub io_number: Option<u32>,
	pub kind: IoRedirectKind,
}

impl Parseable for IoRedirect {
    fn parse(lexer: &mut Lexer) -> Result<Option<IoRedirect>, ParseError> where Self: Sized {
        let io_number = match lexer.peek() {
            Some(token) if token.vocab == Vocabulary::IoNumber => {
                let num = token.representation
                    .parse::<u32>()
                    .map_err(|_| ParseError::InvalidIoNumber(token.representation.clone()))?;
                lexer.next();
                Some(num)
            },
            _ => None,
        };

        if let Some(io_file) = IoFile::parse(lexer)? {
            return Ok(Some(IoRedirect {
                io_number,
                kind: IoRedirectKind::File(io_file),
            }));
        }

        if let Some(io_here) = IoHere::parse(lexer)? {
            return Ok(Some(IoRedirect {
                io_number,
                kind: IoRedirectKind::Here(io_here),
            }));
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::{IoRedirect, IoRedirectKind};
    use crate::lexer::{Lexer, Vocabulary};
    use crate::parser::filename::Filename;
    use crate::parser::here_end::HereEnd;
    use crate::parser::io_file::IoFile;
    use crate::parser::io_here::IoHere;
    use crate::parser::parseable::Parseable;

    fn init_lexer_at_first_token(input: &str) -> Lexer<'_> {
        let mut lexer = Lexer::init(input);
        lexer.next();
        lexer
    }

    #[test]
    fn parses_io_file_redirect_kind() {
        let mut lexer = init_lexer_at_first_token("< in ;");

        let parsed = IoRedirect::parse(&mut lexer).expect("parse should not error");

        assert_eq!(
            parsed,
            Some(IoRedirect {
                io_number: None,
                kind: IoRedirectKind::File(IoFile::Less(Filename("in".to_string()))),
            })
        );
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Semicolon));
    }

    #[test]
    fn parses_io_here_redirect_kind() {
        let mut lexer = init_lexer_at_first_token("<< EOF ;");

        let parsed = IoRedirect::parse(&mut lexer).expect("parse should not error");

        assert_eq!(
            parsed,
            Some(IoRedirect {
                io_number: None,
                kind: IoRedirectKind::Here(IoHere::DLess(HereEnd("EOF".to_string()))),
            })
        );
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Semicolon));
    }

    #[test]
    fn returns_none_for_non_redirect_start_token() {
        let mut lexer = init_lexer_at_first_token("echo");

        let parsed = IoRedirect::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, None);
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Word));
    }

    #[test]
    fn returns_none_when_redirect_target_is_missing() {
        let mut lexer = init_lexer_at_first_token("< ;");

        let parsed = IoRedirect::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, None);
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Semicolon));
    }

    #[test]
    fn returns_none_on_empty_input() {
        let mut lexer = Lexer::init("");
        lexer.next();

        let parsed = IoRedirect::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, None);
        assert_eq!(lexer.peek(), None);
    }

    #[test]
    fn parses_all_io_file_variants() {
        let cases = vec![
            ("<& in ;", IoRedirectKind::File(IoFile::LessAnd(Filename("in".to_string())))),
            ("> out ;", IoRedirectKind::File(IoFile::Greater(Filename("out".to_string())))),
            (">& out ;", IoRedirectKind::File(IoFile::GreatAnd(Filename("out".to_string())))),
            (">> out ;", IoRedirectKind::File(IoFile::DGreat(Filename("out".to_string())))),
            ("<> out ;", IoRedirectKind::File(IoFile::LessGreat(Filename("out".to_string())))),
            (">| out ;", IoRedirectKind::File(IoFile::Clobber(Filename("out".to_string())))),
        ];

        for (input, expected_kind) in cases {
            let mut lexer = init_lexer_at_first_token(input);
            let parsed = IoRedirect::parse(&mut lexer).expect("parse should not error");

            assert_eq!(
                parsed,
                Some(IoRedirect {
                    io_number: None,
                    kind: expected_kind,
                })
            );
            assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Semicolon));
        }
    }

    #[test]
    fn parses_dlessdash_here_redirect_kind() {
        let mut lexer = init_lexer_at_first_token("<<- EOF ;");

        let parsed = IoRedirect::parse(&mut lexer).expect("parse should not error");

        assert_eq!(
            parsed,
            Some(IoRedirect {
                io_number: None,
                kind: IoRedirectKind::Here(IoHere::DLessDash(HereEnd("EOF".to_string()))),
            })
        );
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Semicolon));
    }

    #[test]
    fn returns_none_when_here_end_is_missing() {
        let mut lexer = init_lexer_at_first_token("<< ;");

        let parsed = IoRedirect::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, None);
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Semicolon));
    }

    #[test]
    fn returns_none_for_numeric_word_when_not_classified_as_io_number() {
        let mut lexer = init_lexer_at_first_token("2 < in");

        let parsed = IoRedirect::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, None);
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Word));
    }
}
