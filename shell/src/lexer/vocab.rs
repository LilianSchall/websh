use std::{collections::HashMap, fmt::Display};

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum Vocabulary {
    Word = 0,
    Amperstand, // &
    And, // &&
    Backquote, // `
    Case, // case
    CloseBrace, // }
    CloseParenthese, // )
    CloseParentheseParenthese, // ))
    Do, // do
    DollarOpenParenthese, // $(
    DollarOpenParentheseParenthese, // $((
    Done, // done
    DoubleSemicolon,
    Elif, // elif
    Else, // else
    Esac, // esac
    Fi, // fi
    For, // for
    Heredoc,
    If, // if
    In, // in
    InfInf, // <<
    InfInfMin, // <<-
    IoNumber, // FD to redirect
    Neg, // !
    Newline, // \n
    OpenBrace, // {
    OpenParenthese, // (
    Or, // ||
    RInf, // <
    RInfAnd, // <&
    RInfSup, // <>
    RPipe, // |
    RSup, // >
    RSupAnd, // >&
    RSupPipe, // >|
    RSupSup, // >>
    Semicolon, // ;
    Then, // then
    Until, // until
    While, // while
}

impl Display for Vocabulary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


pub fn map_to_vocab(representation: &str) -> Vocabulary {
    match representation {
        "!"     => Vocabulary::Neg,
        "&"     => Vocabulary::Amperstand,
        "&&"    => Vocabulary::And,
        "("     => Vocabulary::OpenParenthese,
        ")"     => Vocabulary::CloseParenthese,
        ";;"    => Vocabulary::DoubleSemicolon,
        "<"     => Vocabulary::RInf,
        "<&"    => Vocabulary::RInfAnd,
        "<<"    => Vocabulary::InfInf,
        "<<-"   => Vocabulary::InfInfMin,
        "<>"    => Vocabulary::RInfSup,
        ">"     => Vocabulary::RSup,
        ">&"    => Vocabulary::RSupAnd,
        ">>"    => Vocabulary::RSupSup,
        ">|"    => Vocabulary::RSupPipe,
        "\n"    => Vocabulary::Newline,
        "`"     => Vocabulary::Backquote,
        "case"  => Vocabulary::Case,
        "do"    => Vocabulary::Do,
        "done"  => Vocabulary::Done,
        "elif"  => Vocabulary::Elif,
        "else"  => Vocabulary::Else,
        "esac"  => Vocabulary::Esac,
        "fi"    => Vocabulary::Fi,
        "for"   => Vocabulary::For,
        "if"    => Vocabulary::If,
        "in"    => Vocabulary::In,
        "then"  => Vocabulary::Then,
        "until" => Vocabulary::Until,
        "while" => Vocabulary::While,
        "{"     => Vocabulary::OpenBrace,
        "|"     => Vocabulary::RPipe,
        "||"    => Vocabulary::Or,
        "}"     => Vocabulary::CloseBrace,
        ";"     => Vocabulary::Semicolon,
        "$("    => Vocabulary::DollarOpenParenthese,
        "$(("   => Vocabulary::DollarOpenParentheseParenthese,
        "))"    => Vocabulary::CloseParentheseParenthese,
        _       => Vocabulary::Word
    }
}

pub fn is_part_of_operator(representation: String, c: char) -> bool {
    let operators = vec!["$", "$(", "$((", "))", ";", "!", "&", "&&", "(", ")", ";;", "<", "<&", "<<", "<<-", "<>", ">", ">&", ">>", ">|", "\n", "`", "{", "|", "||", "}"];
    let mut new_word: String = representation;
    new_word.push(c);
    operators.contains(&new_word.as_str())
}

pub fn generate_separators() -> Vec<char> {
    vec![' ', '\t', '\n', '\r', ';', '(', ')', '{', '}', '`', '|', '&', '<', '>', '!', '$', '"', '\'']
}
