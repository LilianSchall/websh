use super::filename::Filename;
use crate::parser::parseable::{Parseable, ParseError};
use crate::lexer::Lexer;
use crate::lexer::Vocabulary;

#[derive(Debug, Clone, PartialEq)]
pub enum IoFile {
	Less(Filename),
	LessAnd(Filename),
	Greater(Filename),
	GreatAnd(Filename),
	DGreat(Filename),
	LessGreat(Filename),
	Clobber(Filename),
}

impl Parseable for IoFile {
    fn parse(lexer: &mut Lexer) -> Result<Option<IoFile>, ParseError> where Self: Sized {
        Ok(match lexer.peek() {
            Some(token) if token.vocab == Vocabulary::RInf => {
                lexer.next();
                Filename::parse(lexer)?.map(IoFile::Less)
            },
            Some(token) if token.vocab == Vocabulary::RInfAnd => {
                lexer.next();
                Filename::parse(lexer)?.map(IoFile::LessAnd)
            },
            Some(token) if token.vocab == Vocabulary::RSup => {
                lexer.next();
                Filename::parse(lexer)?.map(IoFile::Greater)
            },
            Some(token) if token.vocab == Vocabulary::RSupAnd => {
                lexer.next();
                Filename::parse(lexer)?.map(IoFile::GreatAnd)
            },
            Some(token) if token.vocab == Vocabulary::RSupSup => {
                lexer.next();
                Filename::parse(lexer)?.map(IoFile::DGreat)
            },
            Some(token) if token.vocab == Vocabulary::RInfSup => {
                lexer.next();
                Filename::parse(lexer)?.map(IoFile::LessGreat)
            },
            Some(token) if token.vocab == Vocabulary::RSupPipe => {
                lexer.next();
                Filename::parse(lexer)?.map(IoFile::Clobber)
            },
            _ => None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::IoFile;
    use crate::lexer::{Lexer, Vocabulary};
    use crate::parser::filename::Filename;
    use crate::parser::parseable::Parseable;

    fn init_lexer_at_first_token(input: &str) -> Lexer<'_> {
        let mut lexer = Lexer::init(input);
        lexer.next();
        lexer
    }

    #[test]
    fn parses_less_redirection() {
        let mut lexer = init_lexer_at_first_token("< in.txt rest");

        let parsed = IoFile::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, Some(IoFile::Less(Filename("in.txt".to_string()))));
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Word));
    }

    #[test]
    fn parses_lessand_redirection() {
        let mut lexer = init_lexer_at_first_token("<& fd");
        let parsed = IoFile::parse(&mut lexer).expect("parse should not error");
        assert_eq!(parsed, Some(IoFile::LessAnd(Filename("fd".to_string()))));
    }

    #[test]
    fn parses_greater_redirection() {
        let mut lexer = init_lexer_at_first_token("> out.txt");
        let parsed = IoFile::parse(&mut lexer).expect("parse should not error");
        assert_eq!(parsed, Some(IoFile::Greater(Filename("out.txt".to_string()))));
    }

    #[test]
    fn parses_greatand_redirection() {
        let mut lexer = init_lexer_at_first_token(">& fd");
        let parsed = IoFile::parse(&mut lexer).expect("parse should not error");
        assert_eq!(parsed, Some(IoFile::GreatAnd(Filename("fd".to_string()))));
    }

    #[test]
    fn parses_dgreat_redirection() {
        let mut lexer = init_lexer_at_first_token(">> append.log");
        let parsed = IoFile::parse(&mut lexer).expect("parse should not error");
        assert_eq!(parsed, Some(IoFile::DGreat(Filename("append.log".to_string()))));
    }

    #[test]
    fn parses_lessgreat_redirection() {
        let mut lexer = init_lexer_at_first_token("<> rw.file");
        let parsed = IoFile::parse(&mut lexer).expect("parse should not error");
        assert_eq!(parsed, Some(IoFile::LessGreat(Filename("rw.file".to_string()))));
    }

    #[test]
    fn parses_clobber_redirection() {
        let mut lexer = init_lexer_at_first_token(">| force.out");
        let parsed = IoFile::parse(&mut lexer).expect("parse should not error");
        assert_eq!(parsed, Some(IoFile::Clobber(Filename("force.out".to_string()))));
    }

    #[test]
    fn returns_none_for_non_io_file_start_token() {
        let mut lexer = init_lexer_at_first_token("echo");

        let parsed = IoFile::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, None);
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Word));
    }

    #[test]
    fn returns_none_when_filename_is_missing_after_operator() {
        let mut lexer = init_lexer_at_first_token("< ;");

        let parsed = IoFile::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, None);
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Semicolon));
    }

    #[test]
    fn returns_none_on_empty_input() {
        let mut lexer = Lexer::init("");
        lexer.next();

        let parsed = IoFile::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, None);
        assert_eq!(lexer.peek(), None);
    }
}
