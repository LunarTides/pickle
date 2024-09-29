use crate::parser::NodeExit;

pub fn compile(root: NodeExit) -> String {
    let mut buffer = String::new();

    buffer.push_str("global _start\n_start:\n");

    buffer.push_str("    mov rax, 60\n");
    buffer.push_str(format!("    mov rdi, {}\n", root.expr.token.value).as_str());
    buffer.push_str("    syscall\n");

    buffer
}
