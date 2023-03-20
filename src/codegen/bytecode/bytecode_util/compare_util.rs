use super::{
    Instruction,
    register_util,
    SizeType,
};

pub fn compare(instruction: Instruction) -> String
{
    let register1 = instruction.get_register(1);
    if register1.is_none()
    {
        panic!("Compare expected at least 1 register");
    }
    let register1 = register1.unwrap();
    let size = instruction.get_size_type();
    let arguments = instruction.get_arguments();
    let register2 = instruction.get_register(2);
    if register2.is_none()
    {
        if arguments.len() < 1
        {
            panic!("Compare expected at least 1 argument");
        }
        let argument = arguments[0].clone();
        let register1 = register_util::get_name(register1.clone(), size);
        return format!("cmp {}, {}\n", register1, argument);
    }
    else
    {
        let register2 = register2.unwrap();
        let register1 = register_util::get_name(register1.clone(), size);
        let register2 = register_util::get_name(register2.clone(), size);
        return format!("cmp {}, {}\n", register1, register2);
    }
}
pub fn set_equal(instruction: Instruction) -> String
{
    let register1 = instruction.get_register(1);
    if register1.is_none()
    {
        panic!("Set equal expected at least 1 register");
    }
    let register1 = register1.unwrap();
    let size = instruction.get_size_type();
    let sete_reg = register_util::get_name(register1.clone(), SizeType::BYTE);
    let register1 = register_util::get_name(register1.clone(), size);   
    return format!("sete {}\nmovzx {}, {}\n", sete_reg, register1, sete_reg);
}