use super::{
    Variable,
    utils
};
pub fn load_int_variable(vars: &Vec<Variable>, name: String) -> String
{
    // println!("load_variable({})", name);
    let value_pos = utils::findvariableindex(&name, &vars);
    if utils::is_argument(&name, &vars)
    {
        return format!("movq -{}(%rbp), %rax\n", value_pos+8);
    }
    return format!("movq -{}(%rbp), %rax\nmovq (%rax), %rax\n", value_pos*8);
}