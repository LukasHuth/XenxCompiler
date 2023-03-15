use super::super::{
    Statement,
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
    if var.statements.len() == 0
    {
        panic!("No value for variable {}", name);
    }
    if var.datatype.array_bounds.len() > 0
    {
        panic!("Arrays not supported yet");
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
        for i in pos..(pos+8)
        {
            used_positions.push(i);
        }
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
    let malloc_code = format!("movq ${}, %rdi\ncall malloc\nsub $8, %rsp\nmovq %rax, -{}(%rbp)\n", size, pos); // TODO: malloc
    let assign = genassignment_old(value, pos, &vars);
    return format!("{}{}", malloc_code, assign);
}
fn genassignment_old(value: &Statement, pos: usize, vars: &Vec<Variable>) -> String
{
    // println!("value: {}", value.to_string());
    let size = utils::get_type_size(value.datatype.clone());
    let expression = utils::parsebinary(value.clone(), &vars);
    if size == 1
    {
        return format!("{}movq -{}(%rbp), %rbx\nmovb %al, (%rbx)\n", expression, pos);
    }
    if size == 2
    {
        return format!("{}movq -{}(%rbp), %rbx\nmovw %ax, (%rbx)\n", expression, pos);
    }
    if size == 4
    {
        return format!("{}movq -{}(%rbp), %rbx\nmovl %eax, (%rbx)\n", expression, pos);
    }
    if size == 8
    {
        return format!("{}movq -{}(%rbp), %rbx\nmovq %rax, (%rbx)\n", expression, pos);
    }
    panic!("Invalid size for assignment ({} bytes)", size);
}