use crate::lexer::{Token, TokenType};

pub enum NodeExprs {
    NodeExprExit(Vec<Token>),
}

impl Default for NodeExprs {
    fn default() -> Self {
        NodeExprs::NodeExprExit(Vec::default())
    }
}

#[derive(Default)]
pub struct NodeStmt {
    pub expr: NodeExprs,
    // pub token: Token,
}

#[derive(Default)]
pub struct NodeRoot {
    pub stmt: NodeStmt,
}

#[derive(Default)]
pub struct Parser {}

impl Parser {
    fn parse_expr(&self, current_token: &Token, iter: &mut std::vec::IntoIter<Token>) -> NodeExprs {
        let next_token = iter.next().expect("Expected token after expression!");

        if current_token.token_type == TokenType::Exit && next_token.token_type == TokenType::Number
        {
            NodeExprs::NodeExprExit(vec![next_token])
        } else {
            panic!("Expected valid number in expression!");
        }
    }

    pub fn parse(&self, tokens: Vec<Token>) -> NodeRoot {
        let mut root = NodeRoot::default();
        let mut root_stmt = NodeStmt::default();

        let mut iter = tokens.into_iter();
        while let Some(token) = iter.next() {
            if token.token_type == TokenType::Exit {
                let expr_node: NodeExprs = self.parse_expr(&token, &mut iter);
                root_stmt.expr = expr_node
            }
        }

        root.stmt = root_stmt;
        root
    }
}
