use crate::codegen::bytecode;
use bytecode::{
    ByteArray,
    Register,
    SizeType,
};

pub fn generate_malloc(bytecode: &mut ByteArray)
{
    bytecode.add_entry("malloc");
    bytecode.add_push_reg(Register::RSI);
    bytecode.add_push_reg(Register::RDI);
    bytecode.add_push_reg(Register::RBP);
    bytecode.add_move_reg_to_reg(Register::RSP, Register::RBP, SizeType::QWORD);
    bytecode.add_move_reg_to_reg(Register::RDI, Register::RSI, SizeType::QWORD); // size
    bytecode.add_move_lit_to_reg("9", Register::RAX, SizeType::QWORD); // SYS_mmap
    bytecode.add_move_lit_to_reg("0", Register::RDI, SizeType::QWORD); // addr
    bytecode.add_move_lit_to_reg("3", Register::RDX, SizeType::QWORD); // prot
    bytecode.add_move_lit_to_reg("34", Register::R10, SizeType::QWORD); // flags
    bytecode.add_move_lit_to_reg("-1", Register::R8, SizeType::QWORD); // fd
    bytecode.add_move_lit_to_reg("0", Register::R9, SizeType::QWORD); // offset
    bytecode.add_syscall();
    bytecode.add_move_reg_to_reg(Register::RBP, Register::RSP, SizeType::QWORD);
    bytecode.add_pop(Register::RBP);
    bytecode.add_pop(Register::RDI);
    bytecode.add_pop(Register::RSI);
    bytecode.add_ret();
}
pub fn generate_free(bytecode: &mut ByteArray)
{
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
}
pub fn generate_exit(bytecode: &mut ByteArray)
{
    bytecode.add_entry("exit");
    bytecode.add_move_lit_to_reg("60", Register::RAX, SizeType::QWORD);
    bytecode.add_syscall();
}
pub fn generate_print(name: &str, bytecode: &mut ByteArray)
{
    let prefix = "std::print_";
    let type_name = &name[prefix.len()..];
    bytecode.add_entry(name);
    bytecode.add_push_reg(Register::RBP);
    bytecode.add_move_reg_to_reg(Register::RSP, Register::RBP, SizeType::QWORD);
    generate_format(type_name, bytecode);
    bytecode.add_move_reg_to_reg(Register::RDI, Register::RSI, SizeType::QWORD);
    bytecode.add_move_reg_to_reg(Register::RAX, Register::RDI, SizeType::QWORD);
    bytecode.add_move_reg_to_reg(Register::RSP, Register::RAX, SizeType::QWORD);
    bytecode.add_move_lit_to_reg("16", Register::RBX, SizeType::QWORD);
    bytecode.add_xor_reg(Register::RDX, Register::RDX, SizeType::QWORD);
    bytecode.add_div(SizeType::QWORD);
    bytecode.add_sub_reg(Register::RSP, Register::RDX, SizeType::QWORD);
    bytecode.add_move_lit_to_reg("0", Register::RAX, SizeType::QWORD);
    bytecode.add_call("printf");
    bytecode.add_move_reg_to_reg(Register::RBP, Register::RSP, SizeType::QWORD);
    bytecode.add_pop(Register::RBP);
    bytecode.add_ret();
}
fn generate_format(type_name: &str, bytecode: &mut ByteArray)
{
    let new_name = format!("format_{}", type_name);
    return bytecode.add_load_constant(&new_name, SizeType::QWORD);
}