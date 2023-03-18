use super::{
    Instruction,
    register_util,
};
pub fn push (instruction: Instruction) -> String
{
    let register1 = instruction.get_register(1);
    let arguments = instruction.get_arguments();
    if register1.is_none() && arguments.len() < 1
    {
        panic!("Push expected 1 register or 1 argument");
    }
    let value: String;
    if register1.is_none()
    {
        value = arguments[0].clone();
    }
    else
    {
        let register1 = register1.unwrap();
        let size = instruction.get_size_type();
        value = register_util::get_name(register1.clone(), size);
    }
    return format!("push {}\n", value);
}
pub fn pop (instruction: Instruction) -> String
{
    let register1 = instruction.get_register(1);
    let size = instruction.get_size_type();
    if register1.is_none()
    {
        panic!("Pop expected 1 register");
    }
    let register1 = register1.unwrap();
    let destination = register_util::get_name(register1.clone(), size);
    return format!("pop {}\n", destination);
}