use crate::codegen::{ByteArray, bytecode::{Register, SizeType}};

use super::{
    Variable,
    utils,
    Datatype,
    super::StatementDatatype,
};
pub fn load_variable(vars: &Vec<Variable>, name: String, datatype: Datatype, bytecode: &mut ByteArray) -> String
{
    // println!("load_variable({})", name);
    let value_pos = utils::findvariableindex(&name, &vars);
    println!("value_pos: {}", value_pos);
    if utils::is_argument(&name, &vars)
    {
        bytecode.add_move_mem_to_reg(Register::RBP, &value_pos.to_string(), Register::RAX, SizeType::QWORD);
        return format!("movq -{}(%rbp), %rax\n", value_pos);
    }
    match datatype.datatype
    {
        StatementDatatype::Int => {
            bytecode.add_move_mem_to_reg(Register::RBP, &value_pos.to_string(), Register::RAX, SizeType::QWORD);
            bytecode.add_move_mem_to_reg(Register::RAX, "0", Register::RAX, SizeType::BYTE);
            return format!("movq -{}(%rbp), %rax\nmovq (%rax), %rax\n", value_pos);
        },
        StatementDatatype::Char | StatementDatatype::Bool => {
            bytecode.add_move_mem_to_reg(Register::RBP, &value_pos.to_string(), Register::RAX, SizeType::QWORD);
            bytecode.add_move_mem_to_reg(Register::RAX, "0", Register::RAX, SizeType::QWORD);
            return format!("movq -{}(%rbp), %rax\nmovb (%rax), %al\n", value_pos);
        },
        StatementDatatype::String =>
        {
            bytecode.add_move_mem_to_reg(Register::RBP, &value_pos.to_string(), Register::RAX, SizeType::QWORD);
            return format!("movq -{}(%rbp), %rax\n", value_pos);
        },
        StatementDatatype::Float => {
            bytecode.add_move_mem_to_reg(Register::RBP, &value_pos.to_string(), Register::RAX, SizeType::QWORD);
            bytecode.add_move_mem_to_reg(Register::RAX, "0", Register::RAX, SizeType::FLOAT);
            return format!("movq -{}(%rbp), %rax\nmovss (%rax), %xmm0\n", value_pos);
        },
        _ => {
            panic!("Unsupported datatype to load into register");
        }
    }
}