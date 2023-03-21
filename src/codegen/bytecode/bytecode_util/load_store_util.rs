use super::{
    Instruction,
    register_util,
};
pub fn load_constant(instruction: Instruction) -> String
{
    let name = instruction.get_arguments();
    if name.len() != 1
    {
        panic!("Invalid number of arguments");
    }
    let name = name[0].clone();
    let register = instruction.get_register(1);
    if register.is_none()
    {
        panic!("Invalid register");
    }
    let register = register.unwrap();
    let size_type = instruction.get_size_type();
    let r1 = register_util::get_name(register, size_type);
    return format!("lea {}, [rel {}]\n", r1, name);
}