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
    let regs = get_register(instruction);
    let set_reg = regs.0;
    let register1 = regs.1;
    return format!("sete {}\nmovzx {}, {}\n", set_reg, register1, set_reg);
}
pub fn set_not_equal(instruction: Instruction) -> String
{
    let regs = get_register(instruction);
    let set_reg = regs.0;
    let register1 = regs.1;
    return format!("setne {}\nmovzx {}, {}\n", set_reg, register1, set_reg);
}
pub fn set_greater(instruction: Instruction) -> String
{
    let regs = get_register(instruction);
    let set_reg = regs.0;
    let register1 = regs.1;
    return format!("setg {}\nmovzx {}, {}\n", set_reg, register1, set_reg);
}
pub fn set_greater_equal(instruction: Instruction) -> String
{
    let regs = get_register(instruction);
    let set_reg = regs.0;
    let register1 = regs.1;
    return format!("setge {}\nmovzx {}, {}\n", set_reg, register1, set_reg);
}
pub fn set_less(instruction: Instruction) -> String
{
    let regs = get_register(instruction);
    let set_reg = regs.0;
    let register1 = regs.1;
    return format!("setl {}\nmovzx {}, {}\n", set_reg, register1, set_reg);
}
pub fn set_less_equal(instruction: Instruction) -> String
{
    let regs = get_register(instruction);
    let set_reg = regs.0;
    let register1 = regs.1;
    return format!("setle {}\nmovzx {}, {}\n", set_reg, register1, set_reg);
}
fn get_register(instruction: Instruction) -> (String, String)
{
    let register1 = instruction.get_register(1);
    if register1.is_none()
    {
        panic!("Set equal expected at least 1 register");
    }
    let register1 = register1.unwrap();
    let size = instruction.get_size_type();
    let set_reg = register_util::get_name(register1.clone(), SizeType::BYTE);
    let register1 = register_util::get_name(register1.clone(), size);   
    return (set_reg, register1);
}
