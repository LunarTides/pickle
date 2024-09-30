use std::vec::IntoIter;

use crate::lexer::{Token, TokenType};

pub enum NodeExprs {
    NodeExprExit(Vec<Token>),
    NodeExprLet(Vec<Token>),
}

impl Default for NodeExprs {
    fn default() -> Self {
        NodeExprs::NodeExprExit(Vec::default())
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
    vars: Vec<String>,
}

impl Parser {
    fn parse_expr(&mut self, current_token: &Token, iter: &mut IntoIter<Token>) -> NodeExprs {
        match current_token.token_type {
            TokenType::Exit => {
                let exit_code = iter
                    .next()
                    .expect("Expected number or identifier after `exit`.");

                if exit_code.token_type != TokenType::Number
                    && exit_code.token_type != TokenType::Identifer
                    && !self.vars.contains(&exit_code.value)
                {
                    panic!(
                        "Expected number or identifier after `exit`. Got {:?}",
                        exit_code.token_type
                    );
                }

                NodeExprs::NodeExprExit(vec![exit_code])
            }
            TokenType::Let => {
                let name = iter.next().expect("Expected identifier after `let`.");
                if name.token_type != TokenType::Identifer {
                    panic!("Expected identifier after `let`.")
                }

                let equals = iter
                    .next()
                    .unwrap_or_else(|| panic!("Expected '=' after `let {}`.", name.value));

                if equals.token_type != TokenType::Operator || equals.value != "=" {
                    panic!("Expected '=' after `let {}`.", name.value);
                }

                let value = iter
                    .next()
                    .unwrap_or_else(|| panic!("Expected value after `let {} = `", name.value));

                if value.token_type != TokenType::Number {
                    panic!("Expected number after `let {} = `.", name.value);
                }

                self.vars.push(name.value.clone());

                NodeExprs::NodeExprLet(vec![name, equals, value])
            }
            TokenType::Number | TokenType::Identifer | TokenType::Operator => unreachable!(),
        }
    }

    pub fn parse(&mut self, tokens: Vec<Token>) -> NodeRoot {
        let mut root = NodeRoot::default();

        let mut iter = tokens.into_iter();
        while let Some(token) = iter.next() {
            let mut stmt = NodeStmt::default();

            let expr_node: NodeExprs = self.parse_expr(&token, &mut iter);
            stmt.expr = expr_node;

            root.stmts.push(stmt);
        }

        root
    }
}
