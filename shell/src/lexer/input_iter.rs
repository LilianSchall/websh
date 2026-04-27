use std::str::Chars;

pub struct InputIterator<'a> {
    input: Chars<'a>,
    len: usize,
    index: usize,
    current_char: Option<char>,
    block_next: bool,
}

impl<'a> InputIterator<'a> {
    pub fn new(input: &'a str) -> Self {
        let chars = input.chars();
        let current_char = None; 
        let len = input.len();
        InputIterator {
            input: chars,
            len,
            index: 0,
            current_char,
            block_next: false,
        }
    }

    pub fn next(&mut self) -> Option<char> {
        if self.block_next {
            self.block_next = false;
            self.index += 1;
            return self.current_char;
        }
        self.current_char = self.input.next();
        self.index += 1;
        self.current_char
    }

    pub fn peek(&self) -> Option<char> {
        self.current_char
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn block_next(&mut self) {
        if self.block_next {
            return;
        }
        self.index -= 1;
        self.block_next = true;
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
