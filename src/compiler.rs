use std::collections::HashMap;

use crate::{
    lexer::{OperatorType, Token, TokenType},
    parser::{Expression, NodeRoot},
};

#[derive(Default)]
pub struct Compiler {
    global_buffer: String,
    data_buffer: String,
    text_buffer: String,

    const_vars: HashMap<String, (String, Option<Token>, i32)>,
    const_var_last_seen_operator: OperatorType,
    const_var_operator_occurrences: HashMap<OperatorType, usize>,
    const_var_chain_index: usize,

    root: NodeRoot,
    index: usize,
}

impl Compiler {
    fn add_const_var(
        &mut self,
        name: String,
        value: String,
        operator: Option<Token>,
        bit_size: i32,
    ) {
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

        self.const_vars.insert(name, (value, operator, bit_size));
    }

    fn compile_expr_as_var(
        &mut self,
        expr: &Expression,
        var_name: String,
        operator: Option<Token>,
    ) {
        if let Some(operator) = operator.clone() {
            let op_type = operator.op_type.unwrap();

            if op_type == self.const_var_last_seen_operator {
                self.const_var_chain_index += 1;
            } else {
                let record = self.const_var_operator_occurrences.get_mut(&op_type);

                if let Some(record) = record {
                    *record += 1;
                } else {
                    self.const_var_operator_occurrences.insert(op_type, 0);
                }

                self.const_var_chain_index = 0;
            }

            self.const_var_last_seen_operator = op_type;
        }

        match expr {
            Expression::Number(token) => {
                let actual_var_name = if let Some(ref op) = operator {
                    let occurrence = self
                        .const_var_operator_occurrences
                        .get(&op.op_type.unwrap())
                        .unwrap_or(&0);

                    format!(
                        "{}_{:?}_{}_{}",
                        var_name,
                        op.op_type.unwrap(),
                        occurrence,
                        self.const_var_chain_index
                    )
                } else {
                    var_name.clone()
                };

                self.add_const_var(actual_var_name, token.value.clone(), operator, 8);
            }
            Expression::BinOp(expr1, op, expr2) => {
                let expr2_op = if let Some(operator) = operator {
                    if op.op_type.unwrap() == OperatorType::Multiply {
                        Some(op.clone())
                    } else {
                        Some(operator)
                    }
                } else {
                    operator
                };

                self.compile_expr_as_var(expr2, var_name.clone(), expr2_op);
                self.compile_expr_as_var(expr1, var_name, Some(op.clone()));
            }
            _ => (),
        }
    }

    fn compile_expr(&mut self, expr: &Expression) {
        macro_rules! add_text {
            ($($args:expr),*) => {
                self.text_buffer.push_str(format!("    {}\n", format!($($args),*)).as_str());
            };
        }

        match expr {
            Expression::Exit(tokens) => {
                let exit_code_token = &tokens.first().unwrap();

                if exit_code_token.token_type == TokenType::Identifer
                    && self
                        .const_vars
                        .keys()
                        .any(|var| var.starts_with(&exit_code_token.value))
                {
                    let mut vars = self
                        .const_vars
                        .keys()
                        .filter(|var| var.starts_with(&exit_code_token.value))
                        .collect::<Vec<_>>();

                    vars.sort();

                    let mut vars = vars.into_iter();

                    let mut last_operator_type = OperatorType::default();
                    let mut chain_index = 0;

                    add_text!("xor rdi, rdi");

                    while let Some(var_name) = vars.next() {
                        let (_, operator, _) = self.const_vars.get(var_name).unwrap();
                        if operator.is_none() {
                            add_text!("mov rdi, [{}]", var_name);
                            break;
                        }

                        let operator = operator.clone().unwrap();

                        if operator.token_type != TokenType::Operator {
                            panic!("Expected operator when compiling.");
                        }

                        if let Some(op_type) = operator.op_type {
                            if op_type == last_operator_type {
                                chain_index += 1;
                            } else {
                                chain_index = 0;

                                if last_operator_type == OperatorType::Multiply {
                                    add_text!("add rdi, rax");
                                }
                            }
                        }

                        last_operator_type = operator.op_type.unwrap_or_default();

                        match operator.op_type {
                            Some(OperatorType::Plus) => {
                                add_text!("add rdi, [{}]", var_name);
                            }
                            Some(OperatorType::Minus) => {
                                add_text!("sub rdi, [{}]", var_name);
                            }
                            Some(OperatorType::Multiply) => {
                                if chain_index == 0 {
                                    add_text!("mov rax, [{}]", var_name);
                                    add_text!("mov rbx, [{}]", vars.next().unwrap());
                                } else {
                                    add_text!("mov rbx, [{}]", var_name);
                                }

                                add_text!("imul rbx");

                                if vars.len() == 0 {
                                    add_text!("add rdi, rax");
                                }
                            }
                            Some(OperatorType::Equals) => unreachable!(),
                            None => {
                                panic!("Invalid operator type.");
                            }
                        };
                    }
                } else {
                    add_text!("mov rdi, {}", exit_code_token.value);
                }

                add_text!("mov rax, 60");
                add_text!("syscall");
            }
            Expression::Let(token, expr) => {
                let name = &token.value;

                match expr.as_ref() {
                    Expression::BinOp(expr1, op, expr2) => {
                        self.compile_expr_as_var(expr1, name.to_string(), Some(op.clone()));
                        self.compile_expr_as_var(expr2, name.to_string(), Some(op.clone()));
                    }
                    Expression::Number(_) => {
                        self.compile_expr_as_var(expr, name.to_string(), None);
                    }
                    _ => (),
                }
            }
            Expression::BinOp(..) | Expression::Number(_) => (),
        }
    }

    pub fn compile(mut self, root: NodeRoot) -> String {
        self.global_buffer = "global _start\n".to_string();

        self.data_buffer = "\nsection .data\n".to_string();
        self.text_buffer = "\nsection .text\n_start:\n".to_string();

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
