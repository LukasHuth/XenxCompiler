pub fn generate_malloc() -> String
{
    let mut data = String::new();
    data.push_str("malloc:\n");
    data.push_str("push %rsi\n");
    data.push_str("push %rdi\n");
    data.push_str("push %rbp\n");
    data.push_str("mov %rsp, %rbp\n");
    data.push_str("mov %rdi, %rsi\n");
    data.push_str("movq $9, %rax\n");
    data.push_str("movq $0, %r9\n");
    data.push_str("movq $-1, %r8\n");
    data.push_str("movq $34, %r10\n");
    data.push_str("movq $3, %rdx\n");
    data.push_str("movq $0, %rdi\n");
    data.push_str("syscall\n");
    data.push_str("mov %rbp, %rsp\n");
    data.push_str("pop %rbp\n");
    data.push_str("pop %rdi\n");
    data.push_str("pop %rsi\n");
    data.push_str("ret\n");
    return data;
}
pub fn generate_free() -> String
{
    let mut data = String::new();
    data.push_str("free:\n");
    data.push_str("push %rsi\n");
    data.push_str("push %rdi\n");
    data.push_str("push %rbp\n");
    data.push_str("mov %rsp, %rbp\n");
    data.push_str("movq $11, %rax\n");
    data.push_str("movq %rdi, %rdi\n");
    data.push_str("movq %rsi, %rsi\n");
    data.push_str("syscall\n");
    data.push_str("cmp $0, %rax\n");
    data.push_str("je .L1\n");
    data.push_str("movq %rax, %rdi\n");
    data.push_str("movq $60, %rax\n");
    data.push_str("syscall\n");
    data.push_str(".L1:\n");
    data.push_str("movq %rbp, %rsp\n");
    data.push_str("popq %rbp\n");
    data.push_str("popq %rdi\n");
    data.push_str("popq %rsi\n");
    data.push_str("ret\n");
    return data;
}