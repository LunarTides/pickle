use crate::parser::{NodeExprs, NodeRoot};

pub fn compile(root: NodeRoot) -> String {
    let mut buffer = String::new();

    match root.stmt.expr {
        NodeExprs::NodeExprExit(token) => {
            buffer.push_str("global _start\n_start:\n");

            buffer.push_str("    mov rax, 60\n");
            buffer.push_str(format!("    mov rdi, {}\n", token[0].value).as_str());
            buffer.push_str("    syscall\n");
        }
    }

    buffer
}
