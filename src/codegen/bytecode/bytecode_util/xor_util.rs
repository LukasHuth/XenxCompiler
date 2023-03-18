use instruction::Instruction;

use crate::codegen::bytecode::instruction;

use super::{
    register_util,
};
pub fn xor_reg_to_reg(instruction: Instruction) -> String {
    let from = instruction.get_register(1);
    let to = instruction.get_register(2);
    if from.is_none() && to.is_none() {
        panic!("Xor expected 2 registers");
    }
    let from = from.unwrap();
    let to = to.unwrap();
    let size = instruction.get_size_type();
    let from = register_util::get_name(from, size);
    let to = register_util::get_name(to, size); 
    return format!("xor {}, {}\n", to, from);
}