use crate::codegen::{ByteArray, bytecode::{Register, SizeType}};

use super::{
    Variable,
    utils,
    Datatype,
    super::StatementDatatype,
};
pub fn load_variable(vars: &Vec<Variable>, name: String, datatype: Datatype, bytecode: &mut ByteArray)
{
    // println!("load_variable({})", name);
    let value_pos = utils::findvariableindex(&name, &vars);
    if utils::is_argument(&name, &vars)
    {
        bytecode.add_move_mem_to_reg(Register::RBP, &value_pos.to_string(), Register::RAX, SizeType::QWORD);
    }
    match datatype.datatype
    {
        StatementDatatype::Int => {
            bytecode.add_move_mem_to_reg(Register::RBP, &value_pos.to_string(), Register::RAX, SizeType::QWORD);
            bytecode.add_move_mem_to_reg(Register::RAX, "0", Register::RAX, SizeType::BYTE);
        },
        StatementDatatype::Char | StatementDatatype::Bool => {
            bytecode.add_move_mem_to_reg(Register::RBP, &value_pos.to_string(), Register::RAX, SizeType::QWORD);
            bytecode.add_move_mem_to_reg(Register::RAX, "0", Register::RAX, SizeType::QWORD);
        },
        StatementDatatype::String =>
        {
            bytecode.add_move_mem_to_reg(Register::RBP, &value_pos.to_string(), Register::RAX, SizeType::QWORD);
        },
        StatementDatatype::Float => {
            bytecode.add_move_mem_to_reg(Register::RBP, &value_pos.to_string(), Register::RAX, SizeType::QWORD);
            bytecode.add_move_mem_to_reg(Register::RAX, "0", Register::RAX, SizeType::FLOAT);
        },
        _ => {
            panic!("Unsupported datatype to load into register");
        }
    }
}