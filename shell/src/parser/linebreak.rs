use super::newline_list::NewlineList;
use crate::parser::parseable::{Parseable, ParseError};
use crate::lexer::Lexer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Linebreak {
	pub newlines: Option<NewlineList>,
}

impl Parseable for Linebreak {
    fn parse(lexer: &mut Lexer) -> Result<Option<Linebreak>, ParseError> where Self: Sized {
        Ok(Some(Linebreak{ newlines: NewlineList::parse(lexer)?}))
    }
}

#[cfg(test)]
mod tests {
    use super::Linebreak;
    use crate::lexer::{Lexer, Vocabulary};
    use crate::parser::newline_list::NewlineList;
    use crate::parser::parseable::Parseable;

    fn init_lexer_at_first_token(input: &str) -> Lexer<'_> {
        let mut lexer = Lexer::init(input);
        lexer.next();
        lexer
    }

    #[test]
    fn parses_linebreak_with_newline_list() {
        let mut lexer = init_lexer_at_first_token("\n\nnext");

        let parsed = Linebreak::parse(&mut lexer).expect("parse should not error");

        assert_eq!(
            parsed,
            Some(Linebreak {
                newlines: Some(NewlineList { count: 2 }),
            })
        );
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Word));
    }

    #[test]
    fn parses_empty_linebreak_on_non_newline_token() {
        let mut lexer = init_lexer_at_first_token("echo");

        let parsed = Linebreak::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, Some(Linebreak { newlines: None }));
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Word));
    }

    #[test]
    fn parses_empty_linebreak_on_empty_input() {
        let mut lexer = Lexer::init("");
        lexer.next();

        let parsed = Linebreak::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, Some(Linebreak { newlines: None }));
        assert_eq!(lexer.peek(), None);
    }
}
