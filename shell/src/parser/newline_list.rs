use crate::parser::parseable::{Parseable, ParseError};
use crate::lexer::vocab::Vocabulary;
use crate::lexer::Lexer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewlineList {
	pub count: usize,
}

impl Parseable for NewlineList {
    fn parse(lexer: &mut Lexer) -> Result<Option<NewlineList>, ParseError> where Self: Sized {
        let mut count = 0;
        while let Some(token) = lexer.peek() {
            println!("peeked token: {:?}", token);
            match token.vocab {
                Vocabulary::Newline => {
                    count += 1;
                    lexer.next();
                },
                _ => break,
            }
        }

        if count > 0 {
            Ok(Some(NewlineList { count }))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NewlineList;
    use crate::lexer::{Lexer, Vocabulary};
    use crate::parser::parseable::Parseable;

    fn init_lexer_at_first_token(input: &str) -> Lexer<'_> {
        let mut lexer = Lexer::init(input);
        lexer.next();
        lexer
    }

    #[test]
    fn parses_single_newline() {
        let mut lexer = init_lexer_at_first_token("\n");

        let parsed = NewlineList::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, Some(NewlineList { count: 1 }));
    }

    #[test]
    fn parses_multiple_newlines_and_stops_on_non_newline() {
        let mut lexer = init_lexer_at_first_token("\n\nnext");

        let parsed = NewlineList::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, Some(NewlineList { count: 2 }));
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Word));
    }

    #[test]
    fn returns_none_when_input_starts_with_non_newline_token() {
        let mut lexer = init_lexer_at_first_token("echo");

        let parsed = NewlineList::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, None);
        assert_eq!(lexer.peek().map(|token| token.vocab), Some(Vocabulary::Word));
    }

    #[test]
    fn returns_none_on_empty_input() {
        let mut lexer = Lexer::init("");
        lexer.next();

        let parsed = NewlineList::parse(&mut lexer).expect("parse should not error");

        assert_eq!(parsed, None);
        assert_eq!(lexer.peek(), None);
    }
}
