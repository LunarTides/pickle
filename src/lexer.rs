use std::borrow::BorrowMut;

#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub enum TokenType {
    #[default]
    Identifer,
    Keyword,
    Number,
    Operator,
    SemiColon,
}

#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub enum OperatorType {
    #[default]
    Plus,
    Minus,
}

#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub enum KeywordType {
    #[default]
    Let,
    Exit,
}

#[derive(Default, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub op_type: Option<OperatorType>,
    pub keyword_type: Option<KeywordType>,
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
                    token.op_type = Some(OperatorType::Plus);
                } else if char == '-' {
                    token.op_type = Some(OperatorType::Minus);
                } else if char != '=' {
                    panic!("Operator '{}' is not yet implemented.", char);
                }

                token.value.push(char);
            } else if char == ';' {
                self.push_token(token, tokens.borrow_mut());
                token = Token::default();

                tokens.push(Token {
                    token_type: TokenType::SemiColon,
                    value: ';'.to_string(),
                    ..Default::default()
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

        token.keyword_type = match token.value.to_lowercase().as_str() {
            "exit" => Some(KeywordType::Exit),
            "let" => Some(KeywordType::Let),
            _ => None,
        };

        if token.keyword_type.is_some() {
            token.token_type = TokenType::Keyword;
        }

        tokens.push(token);
    }
}
