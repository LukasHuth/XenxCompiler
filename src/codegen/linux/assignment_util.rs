use super::super::{
    Statement,
    StatementType,
    StatementDatatype
};
use super::{
    Variable,
    utils
};
pub fn genassignment(statement: Statement, vars: &mut Vec<Variable>, mut used_positions: &mut Vec<usize>, mut highest_position: &mut usize) -> String
{
    // println!("genassignment({})", statement.to_string());
    let var = statement.clone();
    let name = var.name.clone();
    if var.datatype.datatype != StatementDatatype::Int
    {
        panic!("Only int variables are supported for now");
    }
    if var.statements.len() == 0
    {
        panic!("No value for variable {}", name);
    }
    let value = var.statements[0].clone();
    let pos: usize;
    let new: bool;
    if utils::havevariable(&name, &vars)
    {
        pos = utils::findvariableindex(&name, &vars);
        new = false;
    }
    else
    {
        pos = utils::findemptyposition(&mut used_positions, &mut highest_position);
        used_positions.push(pos);
        let var_type = var.datatype.clone();
        vars.push(Variable::new(name.as_str(), pos.clone(), false, var_type));
        new = true;
    }
    // println!("used_positions: {:?}", used_positions.clone());
    if new
    {
        let size = utils::get_type_size(var.datatype.clone());
        return genassignment_new(size, &value, pos, &vars);
    }
    else
    {
        return genassignment_old(&value, pos, &vars);
    }
}
fn genassignment_new(size: i32, value: &Statement, pos: usize, vars: &Vec<Variable>) -> String
{
    let malloc_code = format!("movq ${}, %rdi\ncall malloc\nsub $8, %rsp\nmovq %rax, -{}(%rbp)\n", size, pos*8); // TODO: malloc
    let assign = genassignment_old(value, pos, &vars);
    return format!("{}{}", malloc_code, assign);
}
fn genassignment_old(value: &Statement, pos: usize, vars: &Vec<Variable>) -> String
{
    if value.type_ == StatementType::Literal
    {
        let value = value.name.clone();
        return format!("movq -{}(%rbp), %rax\nmovq ${}, (%rax)\n", pos*8, value);
    }
    // println!("value: {}", value.to_string());
    let expression = utils::parsebinary(value.clone(), &vars);
    return format!("{}movq -{}(%rbp), %rbx\nmovq %rax, (%rbx)\n", expression, pos*8);
}