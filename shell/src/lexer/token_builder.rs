use crate::lexer::Token;

pub struct TokenBuilder {
    pub is_building_operator: bool,
    buffer: String,
}

impl TokenBuilder {

    pub fn new(is_building_operator: bool, buffer: String) -> TokenBuilder {
        TokenBuilder {
            is_building_operator,
            buffer
        }
    }

    pub fn init() -> TokenBuilder {
        TokenBuilder::new(false, String::with_capacity(100))
    }

    pub fn push(&mut self, c: char) {
        self.buffer.push(c);
    }

    pub fn flush(&mut self) -> Option<Token> {
        let token = match self.buffer.is_empty() {
            true => None,
            false => Some(Token::from_str(self.buffer.as_str()))
        };

        self.buffer.clear();
        self.is_building_operator = false;

        token
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn to_string(&self) -> String {
        self.buffer.clone()
    }
}
