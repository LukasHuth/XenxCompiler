use super::{
    Variable,
    utils,
    Datatype,
    super::StatementDatatype,
};
pub fn load_variable(vars: &Vec<Variable>, name: String, datatype: Datatype) -> String
{
    // println!("load_variable({})", name);
    let value_pos = utils::findvariableindex(&name, &vars);
    println!("value_pos: {}", value_pos);
    if utils::is_argument(&name, &vars)
    {
        return format!("movq -{}(%rbp), %rax\n", value_pos);
    }
    match datatype.datatype
    {
        StatementDatatype::Int => {
            return format!("movq -{}(%rbp), %rax\nmovq (%rax), %rax\n", value_pos);
        },
        StatementDatatype::Char | StatementDatatype::Bool => {
            return format!("movq -{}(%rbp), %rax\nmovb (%rax), %al\n", value_pos);
        },
        StatementDatatype::String =>
        {
            return format!("movq -{}(%rbp), %rax\n", value_pos);
        },
        StatementDatatype::Float => {
            return format!("movq -{}(%rbp), %rax\nmovss (%rax), %xmm0\n", value_pos);
        },
        _ => {
            panic!("Unsupported datatype to load into register");
        }
    }
}