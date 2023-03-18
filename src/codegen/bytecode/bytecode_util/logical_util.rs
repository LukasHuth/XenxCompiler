use super::{
    Instruction,
    register_util,
};
pub fn mod_() -> String
{
    format!("mod rax, rbx\n")
}
pub fn and() -> String
{
    format!("and rax, rbx\n")
}
pub fn or() -> String
{
    format!("or rax, rbx\n")
}
pub fn xor() -> String
{
    format!("xor rax, rbx\n")
}
pub fn not() -> String
{
    format!("not rax\n")
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