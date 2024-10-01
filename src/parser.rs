use crate::lexer::{KeywordType, Token, TokenType};

#[derive(Clone)]
pub enum Expression {
    Exit(Vec<Token>),
    Let(Token, Box<Expression>),
    Number(Token),
    BinOp(Box<Expression>, Token, Box<Expression>),
}

impl Default for Expression {
    fn default() -> Self {
        Expression::Exit(Vec::default())
    }
}

#[derive(Default)]
pub struct NodeStmt {
    pub expr: Expression,
}

#[derive(Default)]
pub struct NodeRoot {
    pub stmts: Vec<NodeStmt>,
}

#[derive(Default)]
pub struct Parser {
    tokens: Vec<Token>,
    vars: Vec<String>,
    index: usize,
}

impl Parser {
    fn parse_expr(&mut self, current_token: &Token) -> Option<Expression> {
        // TokenType
        let mut return_value = match current_token.token_type {
            TokenType::Number => {
                if let Some(next_token) = self.peek(1).cloned() {
                    if next_token.token_type == TokenType::Operator {
                        let next_number = self
                            .consume(2)
                            .expect("Expected number after operator.")
                            .clone();

                        if next_number.token_type != TokenType::Number {
                            panic!("Expected number after operator.");
                        }

                        let next_expr = self
                            .parse_expr(&next_number)
                            .expect("Expected number after operator.");

                        return Some(Expression::BinOp(
                            Box::new(next_expr),
                            next_token.clone(),
                            Box::new(Expression::Number(current_token.clone())),
                        ));
                    }
                }

                Some(Expression::Number(current_token.clone()))
            }

            TokenType::Identifer
            | TokenType::Keyword
            | TokenType::Operator
            | TokenType::SemiColon => None,
        };

        if return_value.is_some() {
            return return_value;
        }

        // KeywordType
        return_value = match current_token.keyword_type {
            Some(KeywordType::Exit) => {
                let exit_code = self
                    .consume(1)
                    .expect("Expected number or identifier after `exit`.")
                    .clone();

                if exit_code.token_type != TokenType::Number
                    && exit_code.token_type != TokenType::Identifer
                    && !self.vars.contains(&exit_code.value)
                {
                    panic!(
                        "Expected number or identifier after `exit`. Got {:?}",
                        exit_code.token_type
                    );
                }

                Some(Expression::Exit(vec![exit_code]))
            }
            Some(KeywordType::Let) => {
                let name = self
                    .consume(1)
                    .expect("Expected identifier after `let`.")
                    .clone();

                if name.token_type != TokenType::Identifer {
                    panic!("Expected identifier after `let`.")
                }

                let equals = self
                    .consume(1)
                    .unwrap_or_else(|| panic!("Expected '=' after `let {}`.", name.value))
                    .clone();

                if equals.token_type != TokenType::Operator || equals.value != "=" {
                    panic!("Expected '=' after `let {}`.", name.value);
                }

                self.vars.push(name.value.clone());

                let next_token = self
                    .consume(1)
                    .unwrap_or_else(|| panic!("Expected token after `let {} = `", name.value))
                    .clone();

                let next_expr = self
                    .parse_expr(&next_token)
                    .unwrap_or_else(|| panic!("Expected expression after `let {} = `", name.value));

                Some(Expression::Let(name, Box::new(next_expr)))
            }
            None => None,
        };

        return_value
    }

    pub fn parse(mut self, tokens: Vec<Token>) -> NodeRoot {
        self.tokens.clone_from(&tokens);

        let mut root = NodeRoot::default();

        while let Some(token) = tokens.get(self.index) {
            let mut stmt = NodeStmt::default();

            if let Some(expr_node) = self.parse_expr(token) {
                stmt.expr = expr_node;

                root.stmts.push(stmt);
            }

            self.index += 1;
        }

        root
    }

    fn peek(&self, amount: usize) -> Option<&Token> {
        self.tokens.get(self.index + amount)
    }

    fn consume(&mut self, amount: usize) -> Option<&Token> {
        self.index += amount;
        self.peek(0)
    }
}
