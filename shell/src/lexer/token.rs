use super::vocab::{map_to_vocab, Vocabulary};
use std::fmt;


#[derive(Debug, Clone)]
#[derive(PartialEq)]
pub struct Token {
    pub representation: String,
    pub vocab: Vocabulary,
}

impl Token {
    pub fn new(representation: String, vocab: Vocabulary) -> Self {
        Token {
            representation,
            vocab,
        }
    }

    pub fn from_str(s: &str) -> Token {
        let vocab: Vocabulary = map_to_vocab(s);
        Token::new(s.to_string(), vocab)
    }

    pub fn is_assignment_word(&self) -> bool {
        self.vocab == Vocabulary::Word && self.representation.contains('=')
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Token: {} -> {}", self.representation, self.vocab)
    }
}
