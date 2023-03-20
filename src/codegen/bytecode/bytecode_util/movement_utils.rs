use super::{
    Instruction,
    xor_util,
};
pub fn swap(instruction: Instruction) -> String
{
    let mut result = String::new();
    let register1 = instruction.get_register(1).clone();
    let register2 = instruction.get_register(2).clone();
    if register1.is_none() && register2.is_none()
    {
        panic!("Swap expected at least one register");
    }
    let register1 = register1.unwrap();
    let register2 = register2.unwrap();
    let mut instruction = instruction.clone();
    result += xor_util::xor_reg_to_reg(instruction.clone()).as_str();
    instruction.set_register(register2, 1);
    instruction.set_register(register1, 2);
    result += xor_util::xor_reg_to_reg(instruction.clone()).as_str();
    instruction.set_register(register1, 1);
    instruction.set_register(register2, 2);
    result += xor_util::xor_reg_to_reg(instruction).as_str();
    return result;
}