use super::Variable;
use super::utils;
pub fn load_variable(vars: &Vec<Variable>, name: String) -> String
{
    println!("load_variable({})", name);
    let value_pos = utils::findvariableindex(&name, &vars);
    return format!("movq -{}(%rbp), %rax\nmovq (%rax), %rax\n", value_pos*8);
}