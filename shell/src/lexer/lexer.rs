use crate::lexer::input_iter::InputIterator;
use crate::lexer::token_builder::TokenBuilder;

use super::token::Token;
use super::quoted_mode::QuotedMode;
use super::vocab::{is_part_of_operator};

pub struct Lexer<'a> {
    iter: InputIterator<'a>,
    index: usize,
    current_token: Option<Token>,
    quoted_mode: QuotedMode,
}

impl <'a> Lexer<'a> {
    pub fn new(
        iter: InputIterator<'a>,
        index: usize,
        current_token: Option<Token>,
        quoted_mode: QuotedMode
    ) -> Lexer<'a>
    {
        Lexer { iter, index, current_token, quoted_mode }
    }

    pub fn init(input: &'a str) -> Lexer<'a>
    {
        Lexer::new(InputIterator::new(input), 0, None, QuotedMode::None)
    }

    pub fn peek(&self) -> Option<Token> {
        self.current_token.clone()
    }

    pub fn next(&mut self) -> Option<Token> {
        if self.index >= self.iter.len() {
            return self.current_token.clone();
        }

        let mut builder: TokenBuilder = TokenBuilder::init();

        while self.iter.next().is_some() {
            let token = match self.quoted_mode.is_quoted() {
                true => self.process_quoted(&mut builder),
                false => self.process_unquoted(&mut builder)
            };

            if token.is_some() {
                self.current_token = token.clone();
                return token;
            }
        }

        let token = builder.flush();
        self.current_token = token.clone();
        return token;
    }

    fn process_unquoted(
        &mut self,
        builder: &mut TokenBuilder,
    ) -> Option<Token> {
        let current_char = self.iter.peek().unwrap();
        if is_part_of_operator(builder.to_string(), current_char) {
            builder.is_building_operator = true;
            builder.push(current_char);
            return None;
        }

        // else, if we are building an operator but the current char is not part of it, we need to
        // finalize the operator token and return it
        if builder.is_building_operator {
            let token = builder.flush();
            self.iter.block_next();
            return token;
        }
        else if current_char == '\'' {
            self.quoted_mode = QuotedMode::Single;
            return None;
        }
        else if current_char == '"' {
            self.quoted_mode = QuotedMode::Double;
            return None;
        }
        else if current_char == '\\' {
            self.quoted_mode = QuotedMode::Backslash;
            return None;
        }
        else if is_part_of_operator("".to_string(), current_char) {
            self.iter.block_next();
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
            while let Some(c) = self.iter.next() {
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

    fn process_quoted(
        &mut self,
        builder: &mut TokenBuilder,
    ) -> Option<Token> {

        let current_char = self.iter.peek().unwrap();

        match self.quoted_mode {
            QuotedMode::Backslash => {
                builder.push(current_char);
                self.quoted_mode = QuotedMode::None;
            },
            QuotedMode::Single => {
                if current_char == '\'' {
                    self.quoted_mode = QuotedMode::None;
                } else {
                    builder.push(current_char);
                }
            },
            QuotedMode::Double => {
                if current_char == '"' {
                    self.quoted_mode = QuotedMode::None;
                } else if current_char == '\\' {
                    self.quoted_mode = QuotedMode::Backslash;
                    self.process_quoted(builder);
                    self.quoted_mode = QuotedMode::Double;
                } else {
                    builder.push(current_char);
                }
            }
            QuotedMode::None => unreachable!()
        };
        None
    }
}

