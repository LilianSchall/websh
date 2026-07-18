use std::fmt::Display;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
// R-marked vocabularies are reserved for redirections.
pub enum Vocabulary {
    Word = 0,
    Ampersand,                      // &
    And,                            // &&
    Backquote,                      // `
    Case,                           // case
    CloseBrace,                     // }
    CloseParenthese,                // )
    CloseParentheseParenthese,      // ))
    Do,                             // do
    DollarOpenParenthese,           // $(
    DollarOpenParentheseParenthese, // $((
    Done,                           // done
    DoubleSemicolon,
    Elif, // elif
    Else, // else
    Esac, // esac
    Fi,   // fi
    For,  // for
    Heredoc,
    If,             // if
    In,             // in
    InfInf,         // <<
    InfInfMin,      // <<-
    IoNumber,       // FD to redirect
    Neg,            // !
    Newline,        // \n
    OpenBrace,      // {
    OpenParenthese, // (
    Or,             // ||
    RInf,           // <
    RInfAnd,        // <&
    RInfSup,        // <>
    RPipe,          // |
    RSup,           // >
    RSupAnd,        // >&
    RSupPipe,       // >|
    RSupSup,        // >>
    Semicolon,      // ;
    Then,           // then
    Until,          // until
    While,          // while
}

impl Display for Vocabulary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Vocabulary {
    pub fn is_word(&self) -> bool {
        matches!(
            self,
            Vocabulary::Word
                | Vocabulary::Case
                | Vocabulary::Do
                | Vocabulary::Done
                | Vocabulary::Elif
                | Vocabulary::Else
                | Vocabulary::Esac
                | Vocabulary::Fi
                | Vocabulary::For
                | Vocabulary::If
                | Vocabulary::In
                | Vocabulary::Then
                | Vocabulary::Until
                | Vocabulary::While
        )
    }
}

pub fn map_to_vocab(representation: &str, delimiter_type: Vocabulary) -> Vocabulary {
    match representation {
        "!" => Vocabulary::Neg,
        "&" => Vocabulary::Ampersand,
        "&&" => Vocabulary::And,
        "(" => Vocabulary::OpenParenthese,
        ")" => Vocabulary::CloseParenthese,
        ";;" => Vocabulary::DoubleSemicolon,
        "<" => Vocabulary::RInf,
        "<&" => Vocabulary::RInfAnd,
        "<<" => Vocabulary::InfInf,
        "<<-" => Vocabulary::InfInfMin,
        "<>" => Vocabulary::RInfSup,
        ">" => Vocabulary::RSup,
        ">&" => Vocabulary::RSupAnd,
        ">>" => Vocabulary::RSupSup,
        ">|" => Vocabulary::RSupPipe,
        "\n" => Vocabulary::Newline,
        "`" => Vocabulary::Backquote,
        "case" => Vocabulary::Case,
        "do" => Vocabulary::Do,
        "done" => Vocabulary::Done,
        "elif" => Vocabulary::Elif,
        "else" => Vocabulary::Else,
        "esac" => Vocabulary::Esac,
        "fi" => Vocabulary::Fi,
        "for" => Vocabulary::For,
        "if" => Vocabulary::If,
        "in" => Vocabulary::In,
        "then" => Vocabulary::Then,
        "until" => Vocabulary::Until,
        "while" => Vocabulary::While,
        "{" => Vocabulary::OpenBrace,
        "|" => Vocabulary::RPipe,
        "||" => Vocabulary::Or,
        "}" => Vocabulary::CloseBrace,
        ";" => Vocabulary::Semicolon,
        "$(" => Vocabulary::DollarOpenParenthese,
        "$((" => Vocabulary::DollarOpenParentheseParenthese,
        "))" => Vocabulary::CloseParentheseParenthese,
        ionumber
            if (delimiter_type == Vocabulary::RSup || delimiter_type == Vocabulary::RInf)
                && ionumber.chars().all(|c| c.is_digit(10)) =>
        {
            Vocabulary::IoNumber
        }
        _ => Vocabulary::Word,
    }
}

pub fn get_maybe_operator_type(representation: String, c: char) -> Option<Vocabulary> {
    let operators = vec![
        "$", "$(", "$((", "))", ";", "!", "&", "&&", "(", ")", ";;", "<", "<&", "<<", "<<-", "<>",
        ">", ">&", ">>", ">|", "\n", "`", "{", "|", "||", "}",
    ];
    let mut new_word: String = representation;
    new_word.push(c);
    let is_operator = operators.contains(&new_word.as_str());

    if is_operator {
        Some(map_to_vocab(new_word.as_str(), Vocabulary::Word))
    } else {
        None
    }
}

pub fn generate_separators() -> Vec<char> {
    vec![
        ' ', '\t', '\n', '\r', ';', '(', ')', '{', '}', '`', '|', '&', '<', '>', '!', '$', '"',
        '\'',
    ]
}
