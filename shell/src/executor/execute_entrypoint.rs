use crate::lexer::Lexer;
use crate::parser::{parse, CompleteCommand};

pub fn execute(input: &str) -> String {
    let mut lexer = Lexer::init(input);
    // no token has been consumed yet, so we need to call next() to initialize the first token
    lexer.next();
    while lexer.peek().is_some() {}

    let command: CompleteCommand = parse(&mut lexer).expect("Complete command expected");
    String::from("")
}
