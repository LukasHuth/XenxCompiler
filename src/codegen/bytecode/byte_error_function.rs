use super::{ByteArray, Register, SizeType};

#[derive(Clone, Debug, PartialEq)]
pub enum ErrorFunction
{
    ArrayOutOfBounds,
}
pub fn generate(bytecode: &mut ByteArray, err_func: ErrorFunction)
{
    match err_func
    {
        ErrorFunction::ArrayOutOfBounds => generate_array_out_of_bounds(bytecode),
    }
}
fn generate_array_out_of_bounds(bytecode: &mut ByteArray)
{
    let string = "Error: Tried to access values that are out of bounds of and array!";
    bytecode.add_entry("array_out_of_bounds_except");
    bytecode.add_move_lit_to_reg(&string.len().to_string(), Register::RDI, SizeType::QWORD);
    bytecode.add_call("malloc");
    let mut offset = 0;
    for char in string.chars()
    {
        bytecode.add_move_lit_to_mem(&offset.to_string(), &format!("'{}'", char), Register::RAX, SizeType::BYTE);
        offset += 1;
    }
    bytecode.add_move_reg_to_reg(Register::RAX, Register::RDI, SizeType::QWORD);
    bytecode.add_move_lit_to_reg(&string.len().to_string(), Register::RSI, SizeType::QWORD);
    bytecode.add_comment("aligning the stack to 16bit");
    bytecode.add_move_reg_to_reg(Register::RSP, Register::RAX, SizeType::QWORD);
    bytecode.add_move_lit_to_reg("16", Register::RBX, SizeType::QWORD);
    bytecode.add_xor_reg(Register::RDX, Register::RDX, SizeType::QWORD);
    bytecode.add_div(SizeType::QWORD);
    bytecode.add_sub_reg(Register::RSP, Register::RDX, SizeType::QWORD);
    bytecode.add_move_lit_to_reg("0", Register::RAX, SizeType::QWORD);
    bytecode.add_xor_reg(Register::RAX, Register::RAX, SizeType::QWORD);
    bytecode.add_call("printf");
    bytecode.add_move_lit_to_reg("1", Register::RDX, SizeType::QWORD);
    bytecode.add_call("exit");
}
