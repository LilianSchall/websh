use super::list::List;
use super::and_or::AndOr;
use super::separator_op::SeparatorOp;
use super::separator::Separator;
use super::linebreak::Linebreak;
use super::newline_list::NewlineList;

use crate::lexer::Lexer;
use crate::parser::parseable::{Parseable, ParseError};

#[derive(Debug, Clone, PartialEq)]
pub struct CompleteCommand {
	pub list: List,
	pub separator: Option<Separator>,
}

impl Parseable for crate::parser::complete_command::CompleteCommand {
    fn parse(lexer: &mut Lexer) -> Result<Option<Self>, ParseError> where Self: Sized {
        // 1. Parse the first mandatory AndOr
        let first_and_or = match AndOr::parse(lexer)? {
            Some(ao) => ao,
            None => return Ok(None),
        };

        let mut and_ors = vec![first_and_or];
        let mut sep_ops = vec![];
        let mut final_separator = None;

        // 2. Loop to left-factor List and Separator
        loop {
            // Try to parse a SeparatorOp
            let sep_op = match SeparatorOp::parse(lexer)? {
                Some(op) => op,
                None => {
                    // No ';', but there might be a NewlineList terminating the command
                    if let Some(nl) = NewlineList::parse(lexer)? {
                        final_separator = Some(Separator::NewlineList(nl));
                    }
                    break;
                }
            };

            // We consumed a ';'. Is it a List separator or the final Separator?
            // We find out by trying to parse the next AndOr!
            if let Some(next_and_or) = AndOr::parse(lexer).ok().unwrap_or(None) {
                // It was a List separator. Save both and continue looping.
                sep_ops.push(sep_op);
                and_ors.push(next_and_or);
            } else {
                // It was NOT a List separator. It's the final CompleteCommand terminator!
                // Parse the mandatory Linebreak as defined in your Separator logic
                let linebreak = Linebreak::parse(lexer)?
                    .ok_or_else(|| ParseError::EndOfInput(format!("{}:{}", file!(), line!())))?;
                
                final_separator = Some(Separator::OpLinebreak(sep_op, linebreak));
                break;
            }
        }

        // 3. Build the `List` AST right-associatively to match your original design
        // Your enum: SeparatorOp(Box<List>, SeparatorOp, AndOr) 
        // Example: a ; b ; c -> SeparatorOp( Box( SeparatorOp(Box(c), ;, b) ), ;, a )
        
        let mut and_ors_rev = and_ors.into_iter().rev();
        let mut sep_ops_rev = sep_ops.into_iter().rev();

        // The right-most node is always just an AndOr
        let mut list_ast = List::AndOr(and_ors_rev.next().unwrap());

        // Fold backwards through the remaining items
        for and_or in and_ors_rev {
            let sep_op = sep_ops_rev.next().unwrap();
            list_ast = List::SeparatorOp(Box::new(list_ast), sep_op, and_or);
        }

        Ok(Some(CompleteCommand {
            list: list_ast,
            separator: final_separator,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::CompleteCommand;
    use crate::lexer::{Lexer, Vocabulary};
    use crate::parser::and_or::AndOr;
    use crate::parser::cmd_name::CmdName;
    use crate::parser::cmd_prefix::CmdPrefix;
    use crate::parser::cmd_suffix::CmdSuffix;
    use crate::parser::command::Command;
    use crate::parser::io_file::IoFile;
    use crate::parser::io_here::IoHere;
    use crate::parser::io_redirect::{IoRedirect, IoRedirectKind};
    use crate::parser::list::List;
    use crate::parser::parseable::{Parseable, ParseError};
    use crate::parser::pipe_sequence::PipeSequence;
    use crate::parser::separator::Separator;
    use crate::parser::separator_op::SeparatorOp;

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

    fn collect_io_suffixes(suffix: &CmdSuffix, redirects: &mut Vec<IoRedirect>) {
        match suffix {
            CmdSuffix::WordSuffix(next, _) => {
                if let Some(next_suffix) = next {
                    collect_io_suffixes(next_suffix, redirects);
                }
            }
            CmdSuffix::IoSuffix(next, redirect) => {
                redirects.push(redirect.clone());
                if let Some(next_suffix) = next {
                    collect_io_suffixes(next_suffix, redirects);
                }
            }
        }
    }

    fn count_pipe_commands(sequence: &PipeSequence) -> usize {
        match sequence {
            PipeSequence::Command(_) => 1,
            PipeSequence::Pipe(_, _, next) => 1 + count_pipe_commands(next),
        }
    }

    fn first_command_of_list(list: &List) -> &Command {
        match list {
            List::AndOr(and_or) => match and_or {
                AndOr::Pipeline(pipeline) => match &pipeline.pipe_sequence {
                    PipeSequence::Command(command) => command,
                    PipeSequence::Pipe(command, _, _) => command,
                },
                AndOr::AndIf(_, _, next) => first_command_of_and_or(next),
                AndOr::OrIf(_, _, next) => first_command_of_and_or(next),
            },
            List::SeparatorOp(next, _, _) => first_command_of_list(next),
        }
    }

    fn first_command_of_and_or(and_or: &AndOr) -> &Command {
        match and_or {
            AndOr::Pipeline(pipeline) => match &pipeline.pipe_sequence {
                PipeSequence::Command(command) => command,
                PipeSequence::Pipe(command, _, _) => command,
            },
            AndOr::AndIf(_, _, next) => first_command_of_and_or(next),
            AndOr::OrIf(_, _, next) => first_command_of_and_or(next),
        }
    }

    #[test]
    fn parses_multi_parameter_command_with_trailing_separator() {
        let mut lexer = init_lexer_at_first_token("echo test args1 args2 args3;");

        let parsed = CompleteCommand::parse(&mut lexer).expect("parse should not error");
        let command = parsed.expect("command should parse");

        assert!(matches!(command.separator, Some(Separator::OpLinebreak(SeparatorOp::Semicolon, _))));
        let first_command = first_command_of_list(&command.list);
        match first_command {
            Command::Simple(simple) => {
                assert_eq!(simple.cmd_name, Some(CmdName("echo".to_string())));
                let mut words = Vec::new();
                if let Some(suffix) = &simple.suffixes {
                    collect_word_suffixes(suffix, &mut words);
                }
                assert_eq!(words, vec!["test", "args1", "args2", "args3"]);
            }
            other => panic!("expected simple command, got {other:?}"),
        }
        assert_eq!(lexer.peek(), None);
    }

    #[test]
    fn parses_multi_parameter_command_without_trailing_separator() {
        let mut lexer = init_lexer_at_first_token("echo test args1 args2 args3");

        let parsed = CompleteCommand::parse(&mut lexer).expect("parse should not error");
        let command = parsed.expect("command should parse");

        assert!(matches!(command.separator, None));
        let first_command = first_command_of_list(&command.list);
        match first_command {
            Command::Simple(simple) => {
                assert_eq!(simple.cmd_name, Some(CmdName("echo".to_string())));
                let mut words = Vec::new();
                if let Some(suffix) = &simple.suffixes {
                    collect_word_suffixes(suffix, &mut words);
                }
                assert_eq!(words, vec!["test", "args1", "args2", "args3"]);
            }
            other => panic!("expected simple command, got {other:?}"),
        }
        assert_eq!(lexer.peek(), None);
    }

    #[test]
    fn parses_pipe_chain_commands() {
        let mut lexer = init_lexer_at_first_token("echo a | grep a | wc;");

        let parsed = CompleteCommand::parse(&mut lexer).expect("parse should not error");
        let command = parsed.expect("command should parse");

        match &command.list {
            List::AndOr(AndOr::Pipeline(pipeline)) => {
                assert_eq!(count_pipe_commands(&pipeline.pipe_sequence), 3);
            }
            other => panic!("expected list with single pipeline, got {other:?}"),
        }
        assert!(matches!(command.separator, Some(Separator::OpLinebreak(SeparatorOp::Semicolon, _))));
        assert_eq!(lexer.peek(), None);
    }

    #[test]
    fn parses_env_assignment_prefix_for_executed_command() {
        let mut lexer = init_lexer_at_first_token("A=1 B=2 echo hello world;");

        let parsed = CompleteCommand::parse(&mut lexer).expect("parse should not error");
        let command = parsed.expect("command should parse");
        let first_command = first_command_of_list(&command.list);

        match first_command {
            Command::Simple(simple) => {
                assert!(matches!(
                    simple.prefix,
                    Some(CmdPrefix::AssignmentWordPrefix(_, _))
                ));
                assert_eq!(simple.cmd_word.as_ref().map(|word| word.0.clone()), Some("echo".to_string()));
            }
            other => panic!("expected simple command, got {other:?}"),
        }
        assert_eq!(lexer.peek(), None);
    }

    #[test]
    fn parses_input_output_and_heredoc_redirections() {
        let mut lexer = init_lexer_at_first_token("cat < in.txt > out.txt << EOF;");

        let parsed = CompleteCommand::parse(&mut lexer).expect("parse should not error");
        let command = parsed.expect("command should parse");
        let first_command = first_command_of_list(&command.list);

        match first_command {
            Command::Simple(simple) => {
                let mut redirects = Vec::new();
                if let Some(suffix) = &simple.suffixes {
                    collect_io_suffixes(suffix, &mut redirects);
                }

                assert!(redirects.iter().any(|redirect| {
                    matches!(
                        redirect.kind,
                        IoRedirectKind::File(IoFile::Less(_))
                    )
                }));
                assert!(redirects.iter().any(|redirect| {
                    matches!(
                        redirect.kind,
                        IoRedirectKind::File(IoFile::Greater(_))
                    )
                }));
                assert!(redirects.iter().any(|redirect| {
                    matches!(
                        redirect.kind,
                        IoRedirectKind::Here(IoHere::DLess(_))
                    )
                }));
            }
            other => panic!("expected simple command, got {other:?}"),
        }
        assert_eq!(lexer.peek(), None);
    }

    #[test]
    fn parses_logical_and_or_chain() {
        let mut lexer = init_lexer_at_first_token("echo a && echo b || echo c;");

        let parsed = CompleteCommand::parse(&mut lexer).expect("parse should not error");
        let command = parsed.expect("command should parse");

        match &command.list {
            List::AndOr(AndOr::AndIf(_, _, next)) => {
                assert!(matches!(&**next, AndOr::OrIf(_, _, _)));
            }
            other => panic!("expected and/or chain, got {other:?}"),
        }
        assert_eq!(lexer.peek(), None);
    }

    #[test]
    fn parses_background_separator_between_commands() {
        let mut lexer = init_lexer_at_first_token("echo a & echo b;");

        let parsed = CompleteCommand::parse(&mut lexer).expect("parse should not error");
        let command = parsed.expect("command should parse");

        assert!(matches!(command.list, List::SeparatorOp(_, SeparatorOp::Ampersand, _)));
        assert_eq!(lexer.peek(), None);
    }

    #[test]
    fn parses_bang_pipeline() {
        let mut lexer = init_lexer_at_first_token("! echo ok;");

        let parsed = CompleteCommand::parse(&mut lexer).expect("parse should not error");
        let command = parsed.expect("command should parse");

        match &command.list {
            List::AndOr(AndOr::Pipeline(pipeline)) => {
                assert!(pipeline.bang);
            }
            other => panic!("expected pipeline command, got {other:?}"),
        }
        assert_eq!(lexer.peek(), None);
    }

    #[test]
    fn returns_end_of_input_on_empty_input() {
        let mut lexer = Lexer::init("");
        lexer.next();

        let parsed = CompleteCommand::parse(&mut lexer);

        assert!(matches!(parsed, Err(ParseError::EndOfInput(_))));
    }

    #[test]
    fn returns_end_of_input_on_incomplete_pipe_chain() {
        let mut lexer = init_lexer_at_first_token("echo a | ");

        let parsed = CompleteCommand::parse(&mut lexer);

        assert!(matches!(parsed, Err(ParseError::EndOfInput(_))));
    }

    #[test]
    fn returns_end_of_input_on_incomplete_and_if_chain() {
        let mut lexer = init_lexer_at_first_token("echo a && ");

        let parsed = CompleteCommand::parse(&mut lexer);

        assert!(matches!(parsed, Err(ParseError::EndOfInput(_))));
    }

    #[test]
    fn returns_end_of_input_when_input_starts_with_separator() {
        let mut lexer = init_lexer_at_first_token("; echo hi");

        let parsed = CompleteCommand::parse(&mut lexer);

        assert!(matches!(parsed, Err(ParseError::EndOfInput(_))));
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Semicolon));
    }
}
