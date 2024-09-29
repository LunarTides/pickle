use crate::lexer::{Token, TokenType};

#[derive(Default)]
pub struct NodeExpr {
    pub token: Token
}

#[derive(Default)]
pub struct NodeExit {
    pub expr: NodeExpr,
}

#[derive(Default)]
pub struct Parser {}

impl Parser {
    fn parse_expr(
        &self,
        iter: &mut std::vec::IntoIter<Token>,
    ) -> NodeExpr {
        let next_token = iter.next().expect("Expected token after expression!");

        if next_token.token_type == TokenType::Number || next_token.token_type == TokenType::Identifer {
            return NodeExpr { token: next_token };
        } else {
            panic!("Expected valid number in expression!");
        }
    }

    pub fn parse(&self, tokens: Vec<Token>) -> NodeExit {
        let mut exit_node = NodeExit::default();

        let mut iter = tokens.into_iter();
        while let Some(token) = iter.next() {
            if token.token_type == TokenType::Keyword && token.value == "exit" {
                let expr_node = self.parse_expr(&mut iter);
                exit_node.expr = expr_node;
            }
        }

        exit_node
    }
}
