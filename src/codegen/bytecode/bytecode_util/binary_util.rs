use super::{
    Instruction,
    register_util,
};

pub fn add() -> String
{
    format!("add rax, rbx\n")
}
pub fn sub(instruction: Instruction) -> String
{
    let dest = instruction.get_register(1);
    if dest.is_none()
    {
        panic!("Sub expected at least 1 register");
    }
    let dest = dest.unwrap();
    let dest_name = register_util::get_name(dest, instruction.get_size_type());
    if instruction.get_arguments().len() == 1
    {
        return format!("sub {}, {}\n", dest_name, instruction.get_arguments()[0]);
    }
    let source = instruction.get_register(2);
    if source.is_none()
    {
        panic!("Sub expected at least 2 registers");
    }
    let source = source.unwrap();
    let source_name = register_util::get_name(source, instruction.get_size_type());
    format!("sub {}, {}\n", dest_name, source_name)
}
pub fn mul() -> String
{
    format!("mul rbx\n")
}
pub fn div() -> String
{
    format!("div rbx\n")
}
pub fn shl() -> String
{
    format!("shl rax, rbx\n")
}
pub fn shr() -> String
{
    format!("shr rax, rbx\n")
}