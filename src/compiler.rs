use crate::{
    lexer::TokenType,
    parser::{NodeExprs, NodeRoot},
};

pub fn compile(root: NodeRoot) -> String {
    let mut global_buffer = "global _start\n".to_string();

    let mut data_buffer = "\nsection .data\n".to_string();
    let mut text_buffer = "\nsection .text\n_start:\n".to_string();

    let mut vars = Vec::default();

    for stmt in root.stmts {
        match stmt.expr {
            NodeExprs::NodeExprExit(tokens) => {
                let exit_code_token = &tokens.first().unwrap();

                let exit_code = if exit_code_token.token_type == TokenType::Identifer
                    && vars.contains(&exit_code_token.value)
                {
                    format!("[{}]", exit_code_token.value)
                } else {
                    exit_code_token.value.clone()
                };

                text_buffer.push_str("    mov rax, 60\n");
                text_buffer.push_str(format!("    mov rdi, {}\n", exit_code).as_str());
                text_buffer.push_str("    syscall\n");
            }
            NodeExprs::NodeExprLet(tokens) => {
                let name = &tokens.first().unwrap().value;
                let value = &tokens.get(2).unwrap().value;

                data_buffer.push_str(format!("    {} dq {}\n", name, value).as_str());
                vars.push(name.to_string());
            }
        }
    }

    global_buffer.push_str(&data_buffer);
    global_buffer.push_str(&text_buffer);

    global_buffer
}
