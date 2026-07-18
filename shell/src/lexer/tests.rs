use super::*;

fn lex_all(input: &str) -> Vec<Token> {
    let mut lexer = Lexer::init(input);
    let mut tokens = Vec::new();

    loop {
        match lexer.next() {
            Some(token) => tokens.push(token),
            None => break,
        }
    }

    tokens
}

fn assert_tokens(input: &str, expected: Vec<Token>) {
    let tokens = lex_all(input);
    assert_eq!(tokens, expected);
}

#[test]
fn lexer_newline() {
    assert_tokens(
        "\n",
        vec![Token::new("\n".to_string(), Vocabulary::Newline)],
    );
}

#[test]
fn lexer_multiple_newlines_and_stops_on_non_newline() {
    assert_tokens(
        "\n\nnext",
        vec![
            Token::new("\n".to_string(), Vocabulary::Newline),
            Token::new("\n".to_string(), Vocabulary::Newline),
            Token::new("next".to_string(), Vocabulary::Word),
        ],
    );
}

#[test]
fn lexer_echo_test_semicolon() {
    assert_tokens(
        "echo test;",
        vec![
            Token::new("echo".to_string(), Vocabulary::Word),
            Token::new("test".to_string(), Vocabulary::Word),
            Token::new(";".to_string(), Vocabulary::Semicolon),
        ],
    );
}

#[test]
fn lexer_multiple_words_with_mixed_whitespace() {
    assert_tokens(
        "  echo\tfoo   bar  ",
        vec![
            Token::new("echo".to_string(), Vocabulary::Word),
            Token::new("foo".to_string(), Vocabulary::Word),
            Token::new("bar".to_string(), Vocabulary::Word),
        ],
    );
}

#[test]
fn lexer_newline_is_a_token_separator() {
    assert_tokens(
        "a\nb",
        vec![
            Token::new("a".to_string(), Vocabulary::Word),
            Token::new("\n".to_string(), Vocabulary::Newline),
            Token::new("b".to_string(), Vocabulary::Word),
        ],
    );
}
#[test]
fn lexer_whitespace_is_a_token_separator_followed_by_newline() {
    assert_tokens(
        "a \nb",
        vec![
            Token::new("a".to_string(), Vocabulary::Word),
            Token::new("\n".to_string(), Vocabulary::Newline),
            Token::new("b".to_string(), Vocabulary::Word),
        ],
    );
}

#[test]
fn lexer_lex_io_number_after_word() {
    assert_tokens(
        "echo 2>file",
        vec![
            Token::new("echo".to_string(), Vocabulary::Word),
            Token::new("2".to_string(), Vocabulary::IoNumber),
            Token::new(">".to_string(), Vocabulary::RSup),
            Token::new("file".to_string(), Vocabulary::Word),
        ],
    );
}

#[test]
fn lexer_lex_dont_lex_word_as_ionumber_after_word() {
    assert_tokens(
        "echo test>file",
        vec![
            Token::new("echo".to_string(), Vocabulary::Word),
            Token::new("test".to_string(), Vocabulary::Word),
            Token::new(">".to_string(), Vocabulary::RSup),
            Token::new("file".to_string(), Vocabulary::Word),
        ],
    );
}

#[test]
fn lexer_io_number_at_start_with_greater() {
    assert_tokens(
        "2>file",
        vec![
            Token::new("2".to_string(), Vocabulary::IoNumber),
            Token::new(">".to_string(), Vocabulary::RSup),
            Token::new("file".to_string(), Vocabulary::Word),
        ],
    );
}

#[test]
fn lexer_io_number_at_start_with_less() {
    assert_tokens(
        "0<file",
        vec![
            Token::new("0".to_string(), Vocabulary::IoNumber),
            Token::new("<".to_string(), Vocabulary::RInf),
            Token::new("file".to_string(), Vocabulary::Word),
        ],
    );
}

#[test]
fn lexer_io_number_with_dgreat() {
    assert_tokens(
        "12>>file",
        vec![
            Token::new("12".to_string(), Vocabulary::IoNumber),
            Token::new(">>".to_string(), Vocabulary::RSupSup),
            Token::new("file".to_string(), Vocabulary::Word),
        ],
    );
}

#[test]
fn lexer_io_number_with_greatand() {
    assert_tokens(
        "1>&2",
        vec![
            Token::new("1".to_string(), Vocabulary::IoNumber),
            Token::new(">&".to_string(), Vocabulary::RSupAnd),
            Token::new("2".to_string(), Vocabulary::Word),
        ],
    );
}

#[test]
fn lexer_io_number_with_lessand() {
    assert_tokens(
        "3<&0",
        vec![
            Token::new("3".to_string(), Vocabulary::IoNumber),
            Token::new("<&".to_string(), Vocabulary::RInfAnd),
            Token::new("0".to_string(), Vocabulary::Word),
        ],
    );
}

#[test]
fn lexer_io_number_with_lessgreat() {
    assert_tokens(
        "4<>file",
        vec![
            Token::new("4".to_string(), Vocabulary::IoNumber),
            Token::new("<>".to_string(), Vocabulary::RInfSup),
            Token::new("file".to_string(), Vocabulary::Word),
        ],
    );
}

#[test]
fn lexer_io_number_with_clobber() {
    assert_tokens(
        "5>|file",
        vec![
            Token::new("5".to_string(), Vocabulary::IoNumber),
            Token::new(">|".to_string(), Vocabulary::RSupPipe),
            Token::new("file".to_string(), Vocabulary::Word),
        ],
    );
}

#[test]
fn lexer_io_number_with_dless() {
    assert_tokens(
        "6<<EOF",
        vec![
            Token::new("6".to_string(), Vocabulary::IoNumber),
            Token::new("<<".to_string(), Vocabulary::InfInf),
            Token::new("EOF".to_string(), Vocabulary::Word),
        ],
    );
}

#[test]
fn lexer_io_number_with_dlessdash() {
    assert_tokens(
        "7<<-EOF",
        vec![
            Token::new("7".to_string(), Vocabulary::IoNumber),
            Token::new("<<-".to_string(), Vocabulary::InfInfMin),
            Token::new("EOF".to_string(), Vocabulary::Word),
        ],
    );
}

#[test]
fn lexer_space_between_digits_and_operator_prevents_io_number() {
    assert_tokens(
        "2 >file",
        vec![
            Token::new("2".to_string(), Vocabulary::Word),
            Token::new(">".to_string(), Vocabulary::RSup),
            Token::new("file".to_string(), Vocabulary::Word),
        ],
    );
}

#[test]
fn lexer_io_number_multidigit() {
    assert_tokens(
        "99>file",
        vec![
            Token::new("99".to_string(), Vocabulary::IoNumber),
            Token::new(">".to_string(), Vocabulary::RSup),
            Token::new("file".to_string(), Vocabulary::Word),
        ],
    );
}

#[test]
fn lexer_io_number_with_only_digits_but_less_operator() {
    assert_tokens(
        "42<file",
        vec![
            Token::new("42".to_string(), Vocabulary::IoNumber),
            Token::new("<".to_string(), Vocabulary::RInf),
            Token::new("file".to_string(), Vocabulary::Word),
        ],
    );
}

#[test]
fn lexer_all_basic_operators() {
    assert_tokens(
        "! & && ( ) ;; < <& << <<- <> > >& >> >| ` { | || } ;",
        vec![
            Token::new("!".to_string(), Vocabulary::Neg),
            Token::new("&".to_string(), Vocabulary::Ampersand),
            Token::new("&&".to_string(), Vocabulary::And),
            Token::new("(".to_string(), Vocabulary::OpenParenthese),
            Token::new(")".to_string(), Vocabulary::CloseParenthese),
            Token::new(";;".to_string(), Vocabulary::DoubleSemicolon),
            Token::new("<".to_string(), Vocabulary::RInf),
            Token::new("<&".to_string(), Vocabulary::RInfAnd),
            Token::new("<<".to_string(), Vocabulary::InfInf),
            Token::new("<<-".to_string(), Vocabulary::InfInfMin),
            Token::new("<>".to_string(), Vocabulary::RInfSup),
            Token::new(">".to_string(), Vocabulary::RSup),
            Token::new(">&".to_string(), Vocabulary::RSupAnd),
            Token::new(">>".to_string(), Vocabulary::RSupSup),
            Token::new(">|".to_string(), Vocabulary::RSupPipe),
            Token::new("`".to_string(), Vocabulary::Backquote),
            Token::new("{".to_string(), Vocabulary::OpenBrace),
            Token::new("|".to_string(), Vocabulary::RPipe),
            Token::new("||".to_string(), Vocabulary::Or),
            Token::new("}".to_string(), Vocabulary::CloseBrace),
            Token::new(";".to_string(), Vocabulary::Semicolon),
        ],
    );
}

#[test]
fn lexer_words_around_operators_without_spaces() {
    assert_tokens(
        "echo test;ls&&pwd|cat",
        vec![
            Token::new("echo".to_string(), Vocabulary::Word),
            Token::new("test".to_string(), Vocabulary::Word),
            Token::new(";".to_string(), Vocabulary::Semicolon),
            Token::new("ls".to_string(), Vocabulary::Word),
            Token::new("&&".to_string(), Vocabulary::And),
            Token::new("pwd".to_string(), Vocabulary::Word),
            Token::new("|".to_string(), Vocabulary::RPipe),
            Token::new("cat".to_string(), Vocabulary::Word),
        ],
    );
}

#[test]
fn lexer_single_quoted_string_is_single_word() {
    assert_tokens(
        "echo 'hello world'",
        vec![
            Token::new("echo".to_string(), Vocabulary::Word),
            Token::new("hello world".to_string(), Vocabulary::Word),
        ],
    );
}

#[test]
fn lexer_double_quoted_string_is_single_word() {
    assert_tokens(
        "echo \"hello world\"",
        vec![
            Token::new("echo".to_string(), Vocabulary::Word),
            Token::new("hello world".to_string(), Vocabulary::Word),
        ],
    );
}

#[test]
fn lexer_unclosed_quote_flushes_current_word() {
    assert_tokens(
        "'abc",
        vec![Token::new("abc".to_string(), Vocabulary::Word)],
    );
}

#[test]
fn lexer_backslash_escapes_separator_in_word() {
    assert_tokens(
        "echo foo\\ bar",
        vec![
            Token::new("echo".to_string(), Vocabulary::Word),
            Token::new("foo bar".to_string(), Vocabulary::Word),
        ],
    );
}

#[test]
fn lexer_backslash_escapes_operator_in_word() {
    assert_tokens(
        "foo\\;bar",
        vec![Token::new("foo;bar".to_string(), Vocabulary::Word)],
    );
}

#[test]
fn lexer_comment_is_ignored_until_newline() {
    assert_tokens(
        "echo hi # this is a comment\nnext",
        vec![
            Token::new("echo".to_string(), Vocabulary::Word),
            Token::new("hi".to_string(), Vocabulary::Word),
            Token::new("next".to_string(), Vocabulary::Word),
        ],
    );
}

#[test]
fn lexer_hash_inside_word_is_not_comment() {
    assert_tokens(
        "echo#not_comment",
        vec![Token::new("echo#not_comment".to_string(), Vocabulary::Word)],
    );
}

#[test]
fn lexer_dollar_parenthesis_operators_without_parameter_expansion() {
    assert_tokens(
        "$((x)) $( cmd )",
        vec![
            Token::new(
                "$((".to_string(),
                Vocabulary::DollarOpenParentheseParenthese,
            ),
            Token::new("x".to_string(), Vocabulary::Word),
            Token::new("))".to_string(), Vocabulary::CloseParentheseParenthese),
            Token::new("$(".to_string(), Vocabulary::DollarOpenParenthese),
            Token::new("cmd".to_string(), Vocabulary::Word),
            Token::new(")".to_string(), Vocabulary::CloseParenthese),
        ],
    );
}
