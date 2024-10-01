use std::collections::HashMap;

use crate::{
    lexer::TokenType,
    parser::{Expression, NodeRoot},
};

#[derive(Default)]
pub struct Compiler {
    global_buffer: String,
    data_buffer: String,
    text_buffer: String,

    vars: Vec<String>,
    const_vars: HashMap<String, (String, i32)>,

    root: NodeRoot,
    index: usize,
}

impl Compiler {
    fn add_const_var(&mut self, name: String, value: String, bit_size: i32) {
        let size = match bit_size {
            1 => 'b',
            2 => 'w',
            4 => 'd',
            8 => 'q',
            10 => 't',
            16 => 'o',
            32 => 'y',
            64 => 'z',
            _ => 'q',
        };

        self.data_buffer
            .push_str(format!("    {} d{} {}\n", name, size, value).as_str());

        self.const_vars.insert(name, (value, bit_size));
    }

    fn compile_expr_as_var(&mut self, expr: &Expression, var_name: String, sub_index: usize) {
        match expr {
            Expression::Number(token) => {
                self.add_const_var(var_name, token.value.clone(), 8);
            }
            Expression::BinOp(expr1, _, expr2) => {
                self.compile_expr_as_var(
                    expr1,
                    format!("{}_sub_{}", var_name, sub_index),
                    sub_index + 1,
                );

                self.compile_expr_as_var(
                    expr2,
                    format!("{}_sub_{}", var_name, sub_index + 1),
                    sub_index + 2,
                );
            }
            _ => (),
        }
    }

    fn compile_expr(&mut self, expr: &Expression) {
        let text_buffer = &mut self.text_buffer;

        match expr {
            Expression::Exit(tokens) => {
                let exit_code_token = &tokens.first().unwrap();

                if exit_code_token.token_type == TokenType::Identifer
                    && self
                        .vars
                        .iter()
                        .any(|var| var.starts_with(&exit_code_token.value))
                {
                    let vars = self
                        .const_vars
                        .keys()
                        .filter(|var| var.starts_with(&exit_code_token.value));

                    for (i, var) in vars.enumerate() {
                        if i == 0 {
                            text_buffer.push_str(format!("    mov rdi, [{}]\n", var).as_str());
                            continue;
                        }

                        text_buffer.push_str(format!("    add rdi, [{}]\n", var).as_str());
                    }
                } else {
                    text_buffer
                        .push_str(format!("    mov rdi, {}\n", exit_code_token.value).as_str());
                }

                text_buffer.push_str("    mov rax, 60\n");
                text_buffer.push_str("    syscall\n");
            }
            Expression::Let(token, expr) => {
                let name = &token.value;
                let mut var_name = name.clone();

                match expr.as_ref() {
                    Expression::BinOp(expr1, op, expr2) => {
                        self.compile_expr_as_var(
                            expr1,
                            format!("{}_{:?}_0", name, op.op_type.unwrap()),
                            0,
                        );
                        self.compile_expr_as_var(
                            expr2,
                            format!("{}_{:?}_1", name, op.op_type.unwrap()),
                            0,
                        );

                        var_name = format!("{}_{}", name, op.value);
                    }
                    Expression::Number(_) => {
                        self.compile_expr_as_var(expr, name.to_string(), 0);
                    }
                    _ => (),
                }

                self.vars.push(var_name);
            }
            Expression::BinOp(..) | Expression::Number(_) => (),
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
