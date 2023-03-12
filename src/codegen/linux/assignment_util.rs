use super::super::StatementDatatype;
use super::super::StatementType;
use super::super::Statement;
use super::Variable;
use super::call_util;
use super::utils;
use super::load_util;

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
        vars.push(Variable::new(name.as_str(), pos.clone()));
        new = true;
    }
    println!("used_positions: {:?}", used_positions.clone());
    if new
    {
        return genassignment_new(&value, pos, &vars);
    }
    else
    {
        return genassignment_old(&value, pos, &vars);
    }
}
fn genassignment_new(value: &Statement, pos: usize, vars: &Vec<Variable>) -> String
{
    let malloc_code = format!("movq $8, %rdi\ncall malloc\nsub $8, %rsp\nmovq %rax, -{}(%rbp)\n", pos*8); // TODO: malloc
    let assign = genassignment_old(value, pos, &vars);
    return format!("{}{}", malloc_code, assign);
}
fn genassignment_old(value: &Statement, pos: usize, vars: &Vec<Variable>) -> String
{
    // println!("genassignment_old({})", value.type_ == StatementType::Literal);
    if value.type_ != StatementType::Literal && value.type_ != StatementType::Variable
        && value.type_ != StatementType::Call
    {
        panic!("Only literals and variables are supported for now");
    }
    if value.type_ == StatementType::Variable
    {
        let value = value.name.clone();
        println!("value: {}", value);
        let loaded_value = load_util::load_variable(&vars, value);
        println!("pos: {}", pos);
        println!("loaded_value: '{}'", loaded_value);
        return format!("{}movq -{}(%rbp), %rbx\nmovq %rax, (%rbx)\n", loaded_value, pos*8);
    }
    if value.type_ == StatementType::Call
    {
        let callstr = call_util::gencall(value.clone());
        return format!("{}movq -{}(%rbp), %rbx\nmovq %rax, (%rbx)\n", callstr, pos*8);
    }
    if value.datatype.datatype != StatementDatatype::Int
    {
        panic!("Only int variables are supported for now");
    }
    let value = value.name.clone();
    return format!("movq -{}(%rbp), %rax\nmovq ${}, (%rax)\n", pos*8, value);
}