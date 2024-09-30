use std::borrow::BorrowMut;

#[derive(Default, PartialEq, Clone, Copy)]
pub enum TokenType {
    #[default]
    Identifer,
    Number,
    Exit,
}

#[derive(Default, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

#[derive(Default)]
pub struct Lexer {}

impl Lexer {
    pub fn tokenize(&self, text: String) -> Vec<Token> {
        let mut tokens = vec![];
        let mut token = Token::default();

        for char in text.chars() {
            if char.is_numeric() {
                if token.value.is_empty() {
                    token.token_type = TokenType::Number;
                }

                token.value.push(char);
            } else if char.is_alphabetic() {
                if token.token_type == TokenType::Number {
                    panic!("An identifer / keyword cannot start with a number.");
                }

                token.value.push(char);
            } else {
                self.push_token(token, tokens.borrow_mut());
                token = Token::default();
            }
        }

        if !token.value.is_empty() {
            self.push_token(token, tokens.borrow_mut());
        }

        tokens
    }

    fn push_token(&self, mut token: Token, tokens: &mut Vec<Token>) {
        if token.value.as_str() == "exit" {
            token.token_type = TokenType::Exit;
        }

        tokens.push(token);
    }
}
