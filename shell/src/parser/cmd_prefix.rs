use crate::parser::parseable::{Parseable, ParseError};
use crate::lexer::{Lexer, Vocabulary};

use super::io_redirect::IoRedirect;

#[derive(Debug, Clone, PartialEq)]
pub enum CmdPrefix {
    IoPrefix(Option<Box<CmdPrefix>>, IoRedirect),
    AssignmentWordPrefix(Option<Box<CmdPrefix>>, String),
}

impl Parseable for CmdPrefix {
    fn parse(lexer: &mut Lexer) -> Result<Option<Self>, ParseError> where Self: Sized {
        if let Some(redirect) = IoRedirect::parse(lexer)? {
            let prefix = CmdPrefix::parse(lexer)?.map(Box::from);

            return Ok(Some(CmdPrefix::IoPrefix(prefix, redirect)));
        }

        match lexer.peek() {
            Some(token) if token.is_assignment_word() => {
                lexer.next();
                let prefix = CmdPrefix::parse(lexer)?.map(Box::from);

                return Ok(Some(CmdPrefix::AssignmentWordPrefix(prefix, token.representation)));

            },
            Some(_) =>  Ok(None),
            None => Err(ParseError::EndOfInput)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CmdPrefix;
    use crate::lexer::{Lexer, Vocabulary};
    use crate::parser::filename::Filename;
    use crate::parser::here_end::HereEnd;
    use crate::parser::io_file::IoFile;
    use crate::parser::io_here::IoHere;
    use crate::parser::io_redirect::{IoRedirect, IoRedirectKind};
    use crate::parser::parseable::{Parseable, ParseError};

    fn init_lexer_at_first_token(input: &str) -> Lexer<'_> {
        let mut lexer = Lexer::init(input);
        lexer.next();
        lexer
    }

    #[test]
    fn parses_io_redirect_prefix() {
        let mut lexer = init_lexer_at_first_token("< in.txt ;");

        let parsed = CmdPrefix::parse(&mut lexer).expect("parse should not error");

        assert_eq!(
            parsed,
            Some(CmdPrefix::IoPrefix(
                None,
                IoRedirect {
                    io_number: None,
                    kind: IoRedirectKind::File(IoFile::Less(Filename("in.txt".to_string()))),
                }
            ))
        );
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Semicolon));
    }

    #[test]
    fn returns_none_for_non_prefix_token() {
        let mut lexer = init_lexer_at_first_token(";\n");

        let parsed = CmdPrefix::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, None);
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Semicolon));
    }

    #[test]
    fn returns_end_of_input_error_on_empty_input() {
        let mut lexer = Lexer::init("");
        lexer.next();

        let parsed = CmdPrefix::parse(&mut lexer);

        assert!(matches!(parsed, Err(ParseError::EndOfInput)));
    }

    #[test]
    fn parses_io_here_prefix() {
        let mut lexer = init_lexer_at_first_token("<< EOF ;");

        let parsed = CmdPrefix::parse(&mut lexer).expect("parse should not error");

        assert_eq!(
            parsed,
            Some(CmdPrefix::IoPrefix(
                None,
                IoRedirect {
                    io_number: None,
                    kind: IoRedirectKind::Here(IoHere::DLess(HereEnd("EOF".to_string()))),
                }
            ))
        );
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Semicolon));
    }

    #[test]
    fn parses_chained_io_redirect_prefixes() {
        let mut lexer = init_lexer_at_first_token("< in > out ;");

        let parsed = CmdPrefix::parse(&mut lexer).expect("parse should not error");

        assert_eq!(
            parsed,
            Some(CmdPrefix::IoPrefix(
                Some(Box::new(CmdPrefix::IoPrefix(
                    None,
                    IoRedirect {
                        io_number: None,
                        kind: IoRedirectKind::File(IoFile::Greater(Filename("out".to_string()))),
                    }
                ))),
                IoRedirect {
                    io_number: None,
                    kind: IoRedirectKind::File(IoFile::Less(Filename("in".to_string()))),
                }
            ))
        );
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Semicolon));
    }

    #[test]
    fn returns_end_of_input_error_after_redirect_when_no_followup_token() {
        let mut lexer = init_lexer_at_first_token("< in");

        let parsed = CmdPrefix::parse(&mut lexer);

        assert!(matches!(parsed, Err(ParseError::EndOfInput)));
    }
}
