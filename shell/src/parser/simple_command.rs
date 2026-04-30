use crate::parser::parseable::{Parseable, ParseError};
use crate::lexer::{Lexer};

use super::cmd_name::CmdName;
use super::cmd_prefix::CmdPrefix;
use super::cmd_suffix::CmdSuffix;
use super::cmd_word::CmdWord;

#[derive(Debug, Clone, PartialEq)]
pub struct SimpleCommand {
	pub prefix: Option<CmdPrefix>,
	pub cmd_word: Option<CmdWord>,
	pub cmd_name: Option<CmdName>,
	pub suffixes: Option<CmdSuffix>,
}

impl Parseable for SimpleCommand {
    fn parse(lexer: &mut Lexer) -> Result<Option<Self>, ParseError> where Self: Sized {
        let maybe_prefix = CmdPrefix::parse(lexer)?;

        match maybe_prefix {
            Some(prefix) => parse_full_command(lexer, prefix),
            None  => parse_command_without_prefix(lexer)
        }
    }
}

fn parse_full_command(lexer: &mut Lexer, prefix: CmdPrefix) -> Result<Option<SimpleCommand>, ParseError> {
    let cmd_word = CmdWord::parse(lexer).ok().unwrap_or(None);
    if cmd_word.is_none() {
        return Ok(Some(SimpleCommand { prefix: Some(prefix), cmd_word: None, cmd_name: None, suffixes: None }));
    }

    let suffix = CmdSuffix::parse(lexer).ok().unwrap_or(None);

    Ok(Some(SimpleCommand { prefix: Some(prefix), cmd_word, cmd_name: None, suffixes: suffix }))
}

fn parse_command_without_prefix(lexer: &mut Lexer) -> Result<Option<SimpleCommand>, ParseError> {
    let cmd_name= CmdName::parse(lexer).ok().unwrap_or(None);
    
    if cmd_name.is_none() {
        return Ok(None);
    }

    let suffix = CmdSuffix::parse(lexer).ok().unwrap_or(None);

    Ok(Some(SimpleCommand { prefix: None, cmd_word: None, cmd_name, suffixes: suffix }))
}

#[cfg(test)]
mod tests {
    use super::SimpleCommand;
    use crate::lexer::{Lexer, Vocabulary};
    use crate::parser::cmd_prefix::CmdPrefix;
    use crate::parser::cmd_suffix::CmdSuffix;
    use crate::parser::cmd_word::CmdWord;
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
    fn parses_prefix_only_simple_command_when_cmd_word_is_missing() {
        let mut lexer = init_lexer_at_first_token("< in ;");

        let parsed = SimpleCommand::parse(&mut lexer).expect("parse should not error");

        assert_eq!(
            parsed,
            Some(SimpleCommand {
                prefix: Some(CmdPrefix::IoPrefix(
                    None,
                    IoRedirect {
                        io_number: None,
                        kind: IoRedirectKind::File(IoFile::Less(Filename("in".to_string()))),
                    }
                )),
                cmd_word: None,
                cmd_name: None,
                suffixes: None,
            })
        );
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Semicolon));
    }

    #[test]
    fn returns_none_when_no_prefix_and_no_cmd_name() {
        let mut lexer = init_lexer_at_first_token(";\n");

        let parsed = SimpleCommand::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, None);
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Semicolon));
    }

    #[test]
    fn propagates_end_of_input_error_from_cmd_prefix() {
        let mut lexer = Lexer::init("");
        lexer.next();

        let parsed = SimpleCommand::parse(&mut lexer);

        assert!(matches!(parsed, Err(ParseError::EndOfInput)));
    }

    #[test]
    fn parses_prefix_and_cmd_word_without_suffixes() {
        let mut lexer = init_lexer_at_first_token("< in cmd");

        let parsed = SimpleCommand::parse(&mut lexer).expect("parse should not error");

        assert_eq!(
            parsed,
            Some(SimpleCommand {
                prefix: Some(CmdPrefix::IoPrefix(
                    None,
                    IoRedirect {
                        io_number: None,
                        kind: IoRedirectKind::File(IoFile::Less(Filename("in".to_string()))),
                    }
                )),
                cmd_word: Some(CmdWord("cmd".to_string())),
                cmd_name: None,
                suffixes: None,
            })
        );
        assert_eq!(lexer.peek(), None);
    }

    #[test]
    fn parses_prefix_cmd_word_and_io_suffix() {
        let mut lexer = init_lexer_at_first_token("< in cmd > out ;");

        let parsed = SimpleCommand::parse(&mut lexer).expect("parse should not error");

        assert_eq!(
            parsed,
            Some(SimpleCommand {
                prefix: Some(CmdPrefix::IoPrefix(
                    None,
                    IoRedirect {
                        io_number: None,
                        kind: IoRedirectKind::File(IoFile::Less(Filename("in".to_string()))),
                    }
                )),
                cmd_word: Some(CmdWord("cmd".to_string())),
                cmd_name: None,
                suffixes: Some(CmdSuffix::IoSuffix(
                    None,
                    IoRedirect {
                        io_number: None,
                        kind: IoRedirectKind::File(IoFile::Greater(Filename("out".to_string()))),
                    }
                )),
            })
        );
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Semicolon));
    }

    #[test]
    fn parses_here_doc_prefix_only_command() {
        let mut lexer = init_lexer_at_first_token("<< EOF ;");

        let parsed = SimpleCommand::parse(&mut lexer).expect("parse should not error");

        assert_eq!(
            parsed,
            Some(SimpleCommand {
                prefix: Some(CmdPrefix::IoPrefix(
                    None,
                    IoRedirect {
                        io_number: None,
                        kind: IoRedirectKind::Here(IoHere::DLess(HereEnd("EOF".to_string()))),
                    }
                )),
                cmd_word: None,
                cmd_name: None,
                suffixes: None,
            })
        );
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Semicolon));
    }

    #[test]
    fn propagates_end_of_input_error_after_prefix_without_terminator() {
        let mut lexer = init_lexer_at_first_token("< in");

        let parsed = SimpleCommand::parse(&mut lexer);

        assert!(matches!(parsed, Err(ParseError::EndOfInput)));
    }
}
