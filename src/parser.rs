use crate::lexer::{Token, TokenType};

#[derive(Default)]
pub struct ExprNode {
    // TODO: Change this
    pub number: Token,
}

#[derive(Default)]
pub struct ExitNode {
    pub expr: ExprNode,
}

#[derive(Default)]
pub struct Parser {}

impl Parser {
    fn parse_expr(
        &self,
        iter: &mut std::vec::IntoIter<Token>,
        current_token: &Token,
    ) -> Option<ExprNode> {
        let next_token = iter.next().expect("Expected token after expression!");

        if current_token.token_type == TokenType::Keyword && current_token.value == "exit" {
            if next_token.token_type == TokenType::Number {
                return Some(ExprNode { number: next_token });
            } else {
                panic!("Expected number in `exit`");
            }
        }

        None
    }

    pub fn parse(&self, tokens: Vec<Token>) -> ExitNode {
        let mut exit_node = ExitNode::default();

        let mut iter = tokens.into_iter();
        while let Some(token) = iter.next() {
            if token.token_type == TokenType::Keyword && token.value == "exit" {
                if let Some(expr_node) = self.parse_expr(&mut iter, &token) {
                    exit_node.expr = expr_node;
                } else {
                    panic!("Invalid expression!");
                }
            }
        }

        exit_node
    }
}
