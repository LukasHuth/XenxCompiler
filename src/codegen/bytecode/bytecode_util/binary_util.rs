use super::{
    Instruction,
    register_util,
    get_register_name,
    get_register_names,
};

pub fn add(instruction: Instruction) -> String
{
    let names = get_register_names(instruction);
    let r1 = names.0;
    let r2 = names.1;
    format!("add {}, {}\n", r1, r2)
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
pub fn mul(instruction: Instruction) -> String
{
    let name = get_register_name(instruction);
    format!("mul {}\n", name)
}
pub fn div(instruction: Instruction) -> String
{
    let name = get_register_name(instruction);
    format!("div {}\n", name)
}
pub fn shl(instruction: Instruction) -> String
{
    let names = get_register_names(instruction);
    let r1 = names.0;
    let r2 = names.1;
    format!("shl {}, {}\n", r1, r2)
}
pub fn shr(instruction: Instruction) -> String
{
    let names = get_register_names(instruction);
    let r1 = names.0;
    let r2 = names.1;
    format!("shr {}, {}\n", r1, r2)
}