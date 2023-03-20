use super::{
    Instruction,
};
pub fn jump_equal(instruction: Instruction) -> String
{
    let argument = check_and_get_argument(instruction);
    return format!("je {}\n", argument);
}
pub fn jump_zero(instruction: Instruction) -> String
{
    let argument = check_and_get_argument(instruction);
    return format!("jz {}\n", argument);
}
pub fn jump_greater(instruction: Instruction) -> String
{
    let argument = check_and_get_argument(instruction);
    return format!("jg {}\n", argument);
}
pub fn jump_less(instruction: Instruction) -> String
{
    let argument = check_and_get_argument(instruction);
    return format!("jl {}\n", argument);
}
pub fn jump_greater_equal(instruction: Instruction) -> String
{
    let argument = check_and_get_argument(instruction);
    return format!("jge {}\n", argument);
}
pub fn jump_less_equal(instruction: Instruction) -> String
{
    let argument = check_and_get_argument(instruction);
    return format!("jle {}\n", argument);
}
pub fn jump_not_equal(instruction: Instruction) -> String
{
    let argument = check_and_get_argument(instruction);
    return format!("jne {}\n", argument);
}
pub fn jump_not_zero(instruction: Instruction) -> String
{
    let argument = check_and_get_argument(instruction);
    return format!("jnz {}\n", argument);
}
pub fn jump_negative(instruction: Instruction) -> String
{
    let argument = check_and_get_argument(instruction);
    return format!("jn {}\n", argument);
}
pub fn jump(instruction: Instruction) -> String
{
    let argument = check_and_get_argument(instruction);
    return format!("jmp {}\n", argument);
}
fn check_and_get_argument(instruction: Instruction) -> String
{
    let arguments = instruction.get_arguments();
    if arguments.len() < 1
    {
        panic!("JumpEqual expected 1 argument");
    }
    let argument = arguments[0].clone();
    return argument;
}