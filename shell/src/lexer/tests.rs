use super::*;

#[test]
fn lexer_echo_test_semicolon() {
    let mut lexer = Lexer::init("echo test;");

    lexer = lexer.consume();

    assert_eq!(lexer.current_token, Some(Token::new("echo".to_string(), Vocabulary::Word)));
    lexer = lexer.consume();
    assert_eq!(lexer.current_token, Some(Token::new("test".to_string(), Vocabulary::Word)));
    lexer = lexer.consume();
    assert_eq!(lexer.current_token, Some(Token::new(";".to_string(), Vocabulary::Semicolon)));
    lexer = lexer.consume();
    assert_eq!(lexer.current_token, None);
}
