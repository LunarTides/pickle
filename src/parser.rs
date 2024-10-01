use crate::lexer::{Token, TokenType};

#[derive(Clone)]
pub enum NodeExprs {
    Exit(Vec<Token>),
    Let(Token, Box<NodeExprs>),
    Number(Token),
    Add(Vec<Token>),
}

impl Default for NodeExprs {
    fn default() -> Self {
        NodeExprs::Exit(Vec::default())
    }
}

#[derive(Default)]
pub struct NodeStmt {
    pub expr: NodeExprs,
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
    fn parse_expr(&mut self, current_token: &Token) -> Option<NodeExprs> {
        match current_token.token_type {
            TokenType::Exit => {
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

                Some(NodeExprs::Exit(vec![exit_code]))
            }
            TokenType::Let => {
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

                let values = self
                    .peek_until_semicolon()
                    .unwrap_or_else(|| panic!("Expected expression after `let {} = `", name.value))
                    .to_vec();

                let mut value = None;

                for v in &values {
                    if let Some(expr) = self.parse_expr(v) {
                        match expr {
                            NodeExprs::Add(_) => {
                                value = Some(expr);
                                break;
                            }
                            NodeExprs::Number(_) => {
                                if values
                                    .iter()
                                    .any(|token| token.token_type == TokenType::Plus)
                                {
                                    continue;
                                }

                                value = Some(expr);
                                break;
                            }
                            _ => (),
                        }
                    }
                }

                if value.is_none() {
                    panic!("Expected expression after `let {} = `.", name.value);
                }

                self.vars.push(name.value.clone());

                Some(NodeExprs::Let(name, Box::new(value.unwrap())))
            }
            TokenType::Plus => {
                let tokens: Vec<Token> = self
                    .consume_until_semicolon()
                    .expect("Expected expression after '+'.")
                    .iter()
                    .filter(|token| token.token_type != TokenType::Plus)
                    .cloned()
                    .collect();

                Some(NodeExprs::Add(tokens))
            }
            TokenType::Number => Some(NodeExprs::Number(current_token.clone())),
            TokenType::Identifer | TokenType::Operator | TokenType::SemiColon => None,
        }
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

    fn peek_until_semicolon(&self) -> Option<&[Token]> {
        let first_semicolon_index = self
            .tokens
            .iter()
            .position(|token| token.token_type == TokenType::SemiColon)?;

        self.tokens.get(1 + self.index..first_semicolon_index)
    }

    fn consume_until_semicolon(&mut self) -> Option<&[Token]> {
        let first_semicolon_index = self
            .tokens
            .iter()
            .position(|token| token.token_type == TokenType::SemiColon)?;

        let tokens = self.tokens.get(1 + self.index..first_semicolon_index);
        self.index = first_semicolon_index;
        tokens
    }

    fn consume(&mut self, amount: usize) -> Option<&Token> {
        self.index += amount;
        self.tokens.get(self.index)
    }
}
