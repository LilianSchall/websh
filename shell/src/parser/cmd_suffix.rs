use super::io_redirect::IoRedirect;
use crate::parser::parseable::{Parseable, ParseError};
use crate::lexer::Lexer;
use crate::lexer::Vocabulary;

/* cmd_suffix       :            io_redirect
                 | cmd_suffix io_redirect
                 |            WORD
                 | cmd_suffix WORD
*/

#[derive(PartialEq, Clone, Debug)]
pub enum CmdSuffix {
    IoSuffix(Option<Box<CmdSuffix>>, IoRedirect),
    WordSuffix(Option<Box<CmdSuffix>>, String),
}


impl Parseable for CmdSuffix {
    fn parse(lexer: &mut Lexer) -> Result<Option<Self>, ParseError> where Self: Sized {
        if let Some(redirect) = IoRedirect::parse(lexer)? {
            let suffix = CmdSuffix::parse(lexer)?.map(Box::from);

            return Ok(Some(CmdSuffix::IoSuffix(suffix, redirect)));
        }

        match lexer.peek() {
            Some(token) if token.vocab == Vocabulary::Word => {
                lexer.next();
                let suffix = CmdSuffix::parse(lexer)?.map(Box::from);

                return Ok(Some(CmdSuffix::WordSuffix(suffix, token.representation)));

            },
            Some(_) =>  Ok(None),
            None => Err(ParseError::EndOfInput)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CmdSuffix;
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
    fn parses_io_redirect_suffix() {
        let mut lexer = init_lexer_at_first_token("> out.txt ;");

        let parsed = CmdSuffix::parse(&mut lexer).expect("parse should not error");

        assert_eq!(
            parsed,
            Some(CmdSuffix::IoSuffix(
                None,
                IoRedirect {
                    io_number: None,
                    kind: IoRedirectKind::File(IoFile::Greater(Filename("out.txt".to_string()))),
                }
            ))
        );
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Semicolon));
    }

    #[test]
    fn returns_none_for_non_suffix_token() {
        let mut lexer = init_lexer_at_first_token(";\n");

        let parsed = CmdSuffix::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, None);
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Semicolon));
    }

    #[test]
    fn returns_end_of_input_error_on_empty_input() {
        let mut lexer = Lexer::init("");
        lexer.next();

        let parsed = CmdSuffix::parse(&mut lexer);

        assert!(matches!(parsed, Err(ParseError::EndOfInput)));
    }

    #[test]
    fn parses_io_here_suffix() {
        let mut lexer = init_lexer_at_first_token("<<- EOF ;");

        let parsed = CmdSuffix::parse(&mut lexer).expect("parse should not error");

        assert_eq!(
            parsed,
            Some(CmdSuffix::IoSuffix(
                None,
                IoRedirect {
                    io_number: None,
                    kind: IoRedirectKind::Here(IoHere::DLessDash(HereEnd("EOF".to_string()))),
                }
            ))
        );
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Semicolon));
    }

    #[test]
    fn parses_chained_io_redirect_suffixes() {
        let mut lexer = init_lexer_at_first_token("> out < in ;");

        let parsed = CmdSuffix::parse(&mut lexer).expect("parse should not error");

        assert_eq!(
            parsed,
            Some(CmdSuffix::IoSuffix(
                Some(Box::new(CmdSuffix::IoSuffix(
                    None,
                    IoRedirect {
                        io_number: None,
                        kind: IoRedirectKind::File(IoFile::Less(Filename("in".to_string()))),
                    }
                ))),
                IoRedirect {
                    io_number: None,
                    kind: IoRedirectKind::File(IoFile::Greater(Filename("out".to_string()))),
                }
            ))
        );
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Semicolon));
    }

    #[test]
    fn returns_end_of_input_error_after_redirect_when_no_followup_token() {
        let mut lexer = init_lexer_at_first_token("> out");

        let parsed = CmdSuffix::parse(&mut lexer);

        assert!(matches!(parsed, Err(ParseError::EndOfInput)));
    }
}
