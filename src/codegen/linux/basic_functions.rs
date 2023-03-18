use crate::codegen::bytecode;
use bytecode::{
    ByteArray,
    Register,
    SizeType,
};

pub fn generate_malloc(bytecode: &mut ByteArray) -> String
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
    bytecode.add_entry("malloc");
    bytecode.add_push_reg(Register::RSI);
    bytecode.add_push_reg(Register::RDI);
    bytecode.add_push_reg(Register::RBP);
    bytecode.add_move_reg_to_reg(Register::RSP, Register::RBP, SizeType::QWORD);
    bytecode.add_move_reg_to_reg(Register::RDI, Register::RSI, SizeType::QWORD);
    bytecode.add_move_lit_to_reg("9", Register::RAX, SizeType::QWORD);
    bytecode.add_move_lit_to_reg("0", Register::R9, SizeType::QWORD);
    bytecode.add_move_lit_to_reg("-1", Register::R8, SizeType::QWORD);
    bytecode.add_move_lit_to_reg("34", Register::R10, SizeType::QWORD);
    bytecode.add_move_lit_to_reg("3", Register::RDX, SizeType::QWORD);
    bytecode.add_move_lit_to_reg("0", Register::RDI, SizeType::QWORD);
    bytecode.add_syscall();
    bytecode.add_move_reg_to_reg(Register::RBP, Register::RSP, SizeType::QWORD);
    bytecode.add_pop(Register::RBP);
    bytecode.add_pop(Register::RDI);
    bytecode.add_pop(Register::RSI);
    bytecode.add_ret();
    return data;
}
pub fn generate_free(bytecode: &mut ByteArray) -> String
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
    bytecode.add_entry("free");
    bytecode.add_push_reg(Register::RSI);
    bytecode.add_push_reg(Register::RDI);
    bytecode.add_push_reg(Register::RBP);
    bytecode.add_move_reg_to_reg(Register::RSP, Register::RBP, SizeType::QWORD);
    bytecode.add_move_lit_to_reg("11", Register::RAX, SizeType::QWORD);
    bytecode.add_syscall();
    bytecode.add_cmp_let("0".to_string(), Register::RAX, SizeType::QWORD);
    bytecode.add_jmp_if_eq(".L1");
    bytecode.add_move_reg_to_reg(Register::RAX, Register::RDI, SizeType::QWORD);
    bytecode.add_move_lit_to_reg("60", Register::RAX, SizeType::QWORD);
    bytecode.add_syscall();
    bytecode.add_entry(".L1");
    bytecode.add_move_reg_to_reg(Register::RBP, Register::RSP, SizeType::QWORD);
    bytecode.add_pop(Register::RBP);
    bytecode.add_pop(Register::RDI);
    bytecode.add_pop(Register::RSI);
    bytecode.add_ret();
    return data;
}