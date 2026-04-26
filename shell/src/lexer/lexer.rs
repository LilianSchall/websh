use crate::lexer::input_iter::InputIterator;
use crate::lexer::token_builder::TokenBuilder;

use super::token::Token;
use super::quoted_mode::QuotedMode;
use super::vocab::{is_part_of_operator};

pub struct Lexer<'a> {
    input: &'a str,
    index: usize,
    pub current_token: Option<Token>
}

impl <'a> Lexer<'a> {
    pub fn new(
        input: &'a str,
        index: usize,
        current_token: Option<Token>
    ) -> Lexer<'a>
    {
        Lexer { input, index, current_token }
    }

    pub fn init(input: &'a str) -> Lexer<'a>
    {
        Lexer::new(input, 0, None)
    }

    pub fn consume(&self) -> Lexer<'a> {
        if self.index >= self.input.len() {
            return Lexer::new(self.input, self.index, None);
        }

        let mut quoted_mode: QuotedMode = QuotedMode::None;
        let mut builder: TokenBuilder = TokenBuilder::init();
        println!("Input: {}, index: {}", self.input[self.index..].to_string(), self.index);
        let mut iter = InputIterator::new(&self.input[self.index..]);

        while iter.next().is_some() {
            let token = match quoted_mode.is_quoted() {
                true => process_quoted(&mut builder, &mut iter, &mut quoted_mode),
                false => process_unquoted(&mut builder, &mut iter, &mut quoted_mode)
            };

            if token.is_some() {
                return Lexer::new(self.input, self.index + iter.get_index(), token);
            }
        }

        let token = builder.flush(); 
        Lexer::new(self.input, self.index + iter.get_index(), token)
    }
}

fn process_unquoted(
    builder: &mut TokenBuilder,
    iter: &mut InputIterator,
    quoted_mode: &mut QuotedMode
    ) -> Option<Token> {
    let current_char = iter.peek().unwrap();
    if is_part_of_operator(builder.to_string(), current_char) {
        builder.is_building_operator = true;
        builder.push(current_char);
        return None;
    }

    // else, if we are building an operator but the current char is not part of it, we need to
    // finalize the operator token and return it
    if builder.is_building_operator {
        let token = builder.flush();
        iter.block_next();
        return token;
    }
    else if current_char == '\'' {
        *quoted_mode = QuotedMode::Single;
        return None;
    }
    else if current_char == '"' {
        *quoted_mode = QuotedMode::Double;
        return None;
    }
    else if current_char == '\\' {
        *quoted_mode = QuotedMode::Backslash;
        return None;
    }
    else if is_part_of_operator("".to_string(), current_char) {
        iter.block_next();
        return builder.flush();
    }
    else if current_char.is_whitespace() {
        return builder.flush();
    }
    else if !builder.is_empty(){
        builder.push(current_char);
    }
    else if current_char == '#' {
        let token = builder.flush();
        while let Some(c) = iter.next() {
            if c == '\n' {
                break;
            }
        }
        return token;
    }
    else {
        builder.push(current_char);
    }
    None
}

fn process_quoted<'a>(
    builder: &mut TokenBuilder,
    iter: &mut InputIterator,
    quoted_mode: &mut QuotedMode
    ) -> Option<Token> {

    let current_char = iter.peek().unwrap();

    match quoted_mode {
        QuotedMode::Backslash => {
            builder.push(current_char);
            *quoted_mode = QuotedMode::None;
        },
        QuotedMode::Single => {
            if current_char == '\'' {
                *quoted_mode = QuotedMode::None;
            } else {
                builder.push(current_char);
            }
        },
        QuotedMode::Double => {
            if current_char == '"' {
                *quoted_mode = QuotedMode::None;
            } else if current_char == '\\' {
                *quoted_mode = QuotedMode::Backslash;
                process_quoted(builder, iter, quoted_mode);
                *quoted_mode = QuotedMode::Double;
            } else {
                builder.push(current_char);
            }
        }
        QuotedMode::None => unreachable!()
    };
    None
}
