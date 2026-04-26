use crate::lexer::input_iter::InputIterator;

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
        let mut building_operator: bool = false;
        let mut builder: String = String::with_capacity(100);
        println!("Input: {}, index: {}", self.input[self.index..].to_string(), self.index);
        let mut iter = InputIterator::new(&self.input[self.index..]);

        while let Some(_) = iter.next() {
            println!("Current char: {:?}", iter.peek());
            if !quoted_mode.is_quoted() {
                if let Some(x) = process_unquoted(&mut builder,&mut iter,  &mut building_operator, &mut quoted_mode) {
                    println!("Token found: {}, index: {}", x, iter.get_index());
                    return Lexer::new(self.input, self.index + iter.get_index(), Some(x));
                }
            }
            else {
                /*if let Some(x) = process_quoted(&mut builder, &mut iter, &mut quoted_mode) {
                    return Lexer::new(self.input, iter.get_index(), Some(x));
                }*/
            }
        }

        match builder.is_empty() {
            true => Lexer::new(self.input, self.index + iter.get_index(), None),
            false => {
                let token = Token::from_str(builder.as_str());
                Lexer::new(self.input, self.index + iter.get_index(), Some(token))
            }
        }
    }
}

fn process_unquoted(
    builder: &mut String,
    iter: &mut InputIterator,
    building_operator: &mut bool,
    quoted_mode: &mut QuotedMode
    ) -> Option<Token> {
    let current_char = iter.peek().unwrap();
    if is_part_of_operator(builder.clone(), current_char) {
        *building_operator = true;
        builder.push(current_char);
        return None;
    }

    // else, if we are building an operator but the current char is not part of it, we need to
    // finalize the operator token and return it
    if *building_operator {
        let token = Token::from_str(builder.as_str());
        iter.block_next();
        return Some(token);
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
        if !builder.is_empty() {
            let token = Token::from_str(builder.as_str());
            return Some(token);
        }
    }
    else if current_char.is_whitespace() {
        if !builder.is_empty() {
            let token = Token::from_str(builder.as_str());
            return Some(token);
        }
    }
    else if current_char == '#' {
        if !builder.is_empty() {
            let token = Token::from_str(builder.as_str());
            return Some(token);
        }
        while let Some(c) = iter.next() {
            if c == '\n' {
                break;
            }
        }
        return None;
    }
    else {
        builder.push(current_char);
        println!("Builder: {}", builder);
    }
    None
}

fn process_quoted<'a>(
    builder: &mut String,
    current_char: char,
    quoted_mode: &mut QuotedMode
    ) -> Option<Token> {
    None
}
