use super::{
    Instruction,
    register_util,
    get_register_names,
    get_register_name,
};
pub fn and(instruction: Instruction) -> String
{
    let names = get_register_names(instruction);
    let r1 = names.0;
    let r2 = names.1;
    format!("and {}, {}\n", r1, r2)
}
pub fn or(instruction: Instruction) -> String
{
    let names = get_register_names(instruction);
    let r1 = names.0;
    let r2 = names.1;
    format!("or {}, {}\n", r1, r2)
}
pub fn xor(instruction: Instruction) -> String
{
    let r1 = instruction.get_register(1);
    let r2 = instruction.get_register(2);
    if r1.is_none() || r2.is_none()
    {
        panic!("Xor expected 2 registers");
    }
    let r1 = r1.unwrap();
    let r2 = r2.unwrap();
    let size = instruction.get_size_type();
    let r1 = register_util::get_name(r1, size);
    let r2 = register_util::get_name(r2, size);
    format!("xor {}, {}\n", r1, r2)
}
pub fn not(instruction: Instruction) -> String
{
    let name = get_register_name(instruction);
    format!("not {}\n", name)
}
pub fn neg(instruction: Instruction) -> String
{
    let register = instruction.get_register(1);
    if register.is_none()
    {
        panic!("Neg expected 1 register");
    }
    let register = register.unwrap();
    let size = instruction.get_size_type();
    let register = register_util::get_name(register, size);
    format!("neg {}\n", register)
}