use std::borrow::BorrowMut;

#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub enum TokenType {
    #[default]
    Identifer,
    Number,
    Operator,
    Plus,
    SemiColon,
    Exit,
    Let,
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
            if char.is_alphabetic() {
                if token.token_type == TokenType::Number {
                    panic!("An identifer / keyword cannot start with a number.");
                }

                token.value.push(char);
            } else if char.is_numeric() {
                if token.value.is_empty() {
                    token.token_type = TokenType::Number;
                }

                token.value.push(char);
            } else if ['=', '+', '-', '*', '/'].contains(&char) {
                if char == '=' && token.value.len() > 1 {
                    panic!("Invalid use of '='");
                }

                if char == '=' || token.value.is_empty() {
                    token.token_type = TokenType::Operator;
                }

                if char == '+' {
                    token.token_type = TokenType::Plus;
                }

                token.value.push(char);
            } else if char == ';' {
                self.push_token(token, tokens.borrow_mut());
                token = Token::default();

                tokens.push(Token {
                    token_type: TokenType::SemiColon,
                    value: ';'.to_string(),
                });
            } else {
                self.push_token(token, tokens.borrow_mut());
                token = Token::default();
            }
        }

        self.push_token(token, tokens.borrow_mut());

        tokens
    }

    fn push_token(&self, mut token: Token, tokens: &mut Vec<Token>) {
        if token.value.is_empty() {
            return;
        }

        token.token_type = match token.value.to_lowercase().as_str() {
            "exit" => TokenType::Exit,
            "let" => TokenType::Let,
            _ => token.token_type,
        };

        tokens.push(token);
    }
}
