use crate::lexer::Lexer;
use crate::parser::parseable::{ParseError, Parseable};

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
    fn parse(lexer: &mut Lexer) -> Result<Option<Self>, ParseError>
    where
        Self: Sized,
    {
        let maybe_prefix = CmdPrefix::parse(lexer)?;

        match maybe_prefix {
            Some(prefix) => parse_full_command(lexer, prefix),
            None => parse_command_without_prefix(lexer),
        }
    }
}

fn parse_full_command(
    lexer: &mut Lexer,
    prefix: CmdPrefix,
) -> Result<Option<SimpleCommand>, ParseError> {
    let cmd_word = CmdWord::parse(lexer).ok().unwrap_or(None);
    if cmd_word.is_none() {
        return Ok(Some(SimpleCommand {
            prefix: Some(prefix),
            cmd_word: None,
            cmd_name: None,
            suffixes: None,
        }));
    }

    let suffix = CmdSuffix::parse(lexer).ok().unwrap_or(None);

    Ok(Some(SimpleCommand {
        prefix: Some(prefix),
        cmd_word,
        cmd_name: None,
        suffixes: suffix,
    }))
}

fn parse_command_without_prefix(lexer: &mut Lexer) -> Result<Option<SimpleCommand>, ParseError> {
    let cmd_name = CmdName::parse(lexer).ok().unwrap_or(None);

    if cmd_name.is_none() {
        return Ok(None);
    }

    let suffix = CmdSuffix::parse(lexer).ok().unwrap_or(None);

    Ok(Some(SimpleCommand {
        prefix: None,
        cmd_word: None,
        cmd_name,
        suffixes: suffix,
    }))
}

#[cfg(test)]
mod tests {
    use super::SimpleCommand;
    use crate::lexer::{Lexer, Vocabulary};
    use crate::parser::cmd_name::CmdName;
    use crate::parser::cmd_prefix::CmdPrefix;
    use crate::parser::cmd_suffix::CmdSuffix;
    use crate::parser::cmd_word::CmdWord;
    use crate::parser::filename::Filename;
    use crate::parser::io_file::IoFile;
    use crate::parser::io_redirect::{IoRedirect, IoRedirectKind};
    use crate::parser::parseable::{ParseError, Parseable};

    fn init_lexer_at_first_token(input: &str) -> Lexer<'_> {
        let mut lexer = Lexer::init(input);
        lexer.next();
        lexer
    }

    fn collect_word_suffixes(suffix: &CmdSuffix, words: &mut Vec<String>) {
        match suffix {
            CmdSuffix::WordSuffix(next, word) => {
                words.push(word.clone());
                if let Some(next_suffix) = next {
                    collect_word_suffixes(next_suffix, words);
                }
            }
            CmdSuffix::IoSuffix(next, _) => {
                if let Some(next_suffix) = next {
                    collect_word_suffixes(next_suffix, words);
                }
            }
        }
    }

    fn collect_io_suffixes(suffix: &CmdSuffix, ios: &mut Vec<IoRedirect>) {
        match suffix {
            CmdSuffix::WordSuffix(next, _) => {
                if let Some(next_suffix) = next {
                    collect_io_suffixes(next_suffix, ios);
                }
            }
            CmdSuffix::IoSuffix(next, io_redirect) => {
                ios.push(io_redirect.clone());
                if let Some(next_suffix) = next {
                    collect_io_suffixes(next_suffix, ios);
                }
            }
        }
    }

    fn word_suffixes_from(command: &SimpleCommand) -> Vec<String> {
        let mut words = Vec::new();
        if let Some(suffix) = &command.suffixes {
            collect_word_suffixes(suffix, &mut words);
        }
        words
    }

    fn io_suffixes_from(command: &SimpleCommand) -> Vec<IoRedirect> {
        let mut ios = Vec::new();
        if let Some(suffix) = &command.suffixes {
            collect_io_suffixes(suffix, &mut ios);
        }
        ios
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
        assert_eq!(
            lexer.peek().map(|token| token.vocab),
            Some(Vocabulary::Semicolon)
        );
    }

    #[test]
    fn returns_none_when_no_prefix_and_no_cmd_name() {
        let mut lexer = init_lexer_at_first_token(";\n");

        let parsed = SimpleCommand::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, None);
        assert_eq!(
            lexer.peek().map(|token| token.vocab),
            Some(Vocabulary::Semicolon)
        );
    }

    #[test]
    fn parses_plain_command_with_many_arguments() {
        let mut lexer = init_lexer_at_first_token("echo test args1 args2 args3;");

        let parsed = SimpleCommand::parse(&mut lexer).expect("parse should not error");
        let command = parsed.expect("command should parse");

        assert_eq!(command.prefix, None);
        assert_eq!(command.cmd_word, None);
        assert_eq!(command.cmd_name, Some(CmdName("echo".to_string())));
        assert_eq!(
            word_suffixes_from(&command),
            vec!["test", "args1", "args2", "args3"]
        );
        assert_eq!(
            lexer.peek().map(|token| token.vocab),
            Some(Vocabulary::Semicolon)
        );
    }

    #[test]
    fn parses_command_name_only_without_suffixes() {
        let mut lexer = init_lexer_at_first_token("echo");

        let parsed = SimpleCommand::parse(&mut lexer).expect("parse should not error");

        assert_eq!(
            parsed,
            Some(SimpleCommand {
                prefix: None,
                cmd_word: None,
                cmd_name: Some(CmdName("echo".to_string())),
                suffixes: None,
            })
        );
        assert_eq!(lexer.peek(), None);
    }

    #[test]
    fn parses_plain_command_with_mixed_word_and_io_suffixes() {
        let mut lexer = init_lexer_at_first_token("echo test > out.txt args2 ;");

        let parsed = SimpleCommand::parse(&mut lexer).expect("parse should not error");
        let command = parsed.expect("command should parse");

        assert_eq!(command.prefix, None);
        assert_eq!(command.cmd_name, Some(CmdName("echo".to_string())));
        assert_eq!(word_suffixes_from(&command), vec!["test", "args2"]);
        assert_eq!(
            io_suffixes_from(&command),
            vec![IoRedirect {
                io_number: None,
                kind: IoRedirectKind::File(IoFile::Greater(Filename("out.txt".to_string()))),
            }]
        );
        assert_eq!(
            lexer.peek().map(|token| token.vocab),
            Some(Vocabulary::Semicolon)
        );
    }

    #[test]
    fn parses_assignment_prefix_then_command_and_arguments() {
        let mut lexer = init_lexer_at_first_token("A=1 echo hello world ;");

        let parsed = SimpleCommand::parse(&mut lexer).expect("parse should not error");
        let command = parsed.expect("command should parse");

        assert_eq!(
            command.prefix,
            Some(CmdPrefix::AssignmentWordPrefix(None, "A=1".to_string()))
        );
        assert_eq!(command.cmd_word, Some(CmdWord("echo".to_string())));
        assert_eq!(command.cmd_name, None);
        assert_eq!(word_suffixes_from(&command), vec!["hello", "world"]);
        assert_eq!(
            lexer.peek().map(|token| token.vocab),
            Some(Vocabulary::Semicolon)
        );
    }

    #[test]
    fn propagates_end_of_input_error_from_cmd_prefix() {
        let mut lexer = Lexer::init("");
        lexer.next();

        let parsed = SimpleCommand::parse(&mut lexer);

        assert!(matches!(parsed, Err(ParseError::EndOfInput(_))));
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
        assert_eq!(
            lexer.peek().map(|token| token.vocab),
            Some(Vocabulary::Semicolon)
        );
    }

    #[test]
    fn returns_none_for_invalid_start_token_without_consuming() {
        let mut lexer = init_lexer_at_first_token("&& echo");

        let parsed = SimpleCommand::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, None);
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::And));
    }

    #[test]
    fn returns_prefix_only_when_input_ends_after_prefix() {
        let mut lexer = init_lexer_at_first_token("< in");

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
        assert_eq!(lexer.peek(), None);
    }
}
