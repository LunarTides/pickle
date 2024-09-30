use std::collections::HashMap;

use crate::{
    lexer::TokenType,
    parser::{NodeExprs, NodeRoot},
};

#[derive(Default)]
pub struct Compiler {
    global_buffer: String,
    data_buffer: String,
    text_buffer: String,

    vars: HashMap<String, NodeExprs>,

    root: NodeRoot,
    index: usize,
}

impl Compiler {
    fn compile_expr(&mut self, expr: &NodeExprs) {
        let text_buffer = &mut self.text_buffer;
        let data_buffer = &mut self.data_buffer;

        match expr {
            NodeExprs::Exit(tokens) => {
                let exit_code_token = &tokens.first().unwrap();

                if exit_code_token.token_type == TokenType::Identifer
                    && self.vars.contains_key(&exit_code_token.value)
                {
                    let name = &exit_code_token.value;
                    let var = self.vars.get(name).unwrap();

                    match var {
                        NodeExprs::Add(tokens) => {
                            text_buffer
                                .push_str(format!("    mov rdi, [{}_add_0]\n", name).as_str());

                            for (i, _) in tokens
                                .iter()
                                .filter(|token| token.token_type == TokenType::Number)
                                .enumerate()
                            {
                                if i == 0 {
                                    continue;
                                }

                                text_buffer.push_str(
                                    format!("    add rdi, [{}_add_{}]\n", name, i).as_str(),
                                );
                            }
                        }
                        NodeExprs::Number(_) => {
                            text_buffer.push_str(format!("    mov rdi, [{}]\n", name).as_str());
                        }
                        _ => unreachable!(),
                    }
                } else {
                    text_buffer
                        .push_str(format!("    mov rdi, {}\n", exit_code_token.value).as_str());
                }

                text_buffer.push_str("    mov rax, 60\n");
                text_buffer.push_str("    syscall\n");
            }
            NodeExprs::Let(token, expr) => {
                let name = &token.value;

                match expr.as_ref() {
                    NodeExprs::Add(tokens) => {
                        for (i, token) in tokens
                            .iter()
                            .filter(|token| token.token_type == TokenType::Number)
                            .enumerate()
                        {
                            data_buffer.push_str(
                                format!("    {}_add_{} dq {}\n", name, i, token.value).as_str(),
                            );
                        }
                    }
                    NodeExprs::Number(token) => {
                        data_buffer.push_str(format!("    {} dq {}\n", name, token.value).as_str());
                    }
                    _ => (),
                }

                self.vars.insert(name.to_string(), expr.as_ref().clone());
            }
            NodeExprs::Add(_) | NodeExprs::Number(_) => (),
        }
    }

    pub fn compile(mut self, root: NodeRoot) -> String {
        self.global_buffer = "global _start\n".to_string();

        self.data_buffer = "\nsection .data\n".to_string();
        self.text_buffer = "\nsection .text\n_start:\n".to_string();

        self.vars.clear();

        self.root = root;
        self.index = 0;

        while let Some(stmt) = self.root.stmts.get(self.index) {
            self.compile_expr(&stmt.expr.clone());
            self.index += 1;
        }

        self.global_buffer.push_str(&self.data_buffer);
        self.global_buffer.push_str(&self.text_buffer);

        self.global_buffer
    }
}
