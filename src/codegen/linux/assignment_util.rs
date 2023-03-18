use crate::codegen::ByteArray;
use crate::codegen::bytecode::{
    Register,
    SizeType,
};

use super::super::{
    Statement,
    StatementDatatype,
};
use super::{
    Variable,
    utils
};
pub fn genassignment(statement: Statement, vars: &mut Vec<Variable>, mut used_positions: &mut Vec<usize>, mut highest_position: &mut usize, bytecode: &mut ByteArray) -> String
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
        if var_type.datatype == StatementDatatype::String
        {
            vars.push(Variable::new_string(name.as_str(), pos.clone(), true, var_type));
        }
        else
        {
            vars.push(Variable::new(name.as_str(), pos.clone(), false, var_type));
        }
        new = true;
    }
    // println!("used_positions: {:?}", used_positions.clone());
    if new
    {
        let size = utils::get_type_size(var.datatype.clone());
        return genassignment_new(size, &value, pos, &vars, bytecode);
    }
    else
    {
        return genassignment_old(&value, pos, &vars, bytecode);
    }
}
fn genassignment_new(size: i32, value: &Statement, pos: usize, vars: &Vec<Variable>, bytecode: &mut ByteArray) -> String
{
    let mut size = size;
    if value.datatype.datatype == StatementDatatype::String
    {
        let expression = value.name.clone();
        size = expression.len() as i32 - 2;
    }
    let malloc_code = format!("movq ${}, %rdi\ncall malloc\nsub $8, %rsp\nmovq %rax, -{}(%rbp)\n", size, pos); // TODO: malloc
    bytecode.add_move_lit_to_reg(&size.to_string(), Register::RDI, SizeType::QWORD);
    bytecode.add_call("malloc");
    bytecode.add_move_lit_to_reg("8", Register::RBX, SizeType::QWORD);
    bytecode.add_sub_lit_reg(&size.to_string(), Register::RSP, SizeType::QWORD);
    let assign = genassignment_old(value, pos, &vars, bytecode);
    return format!("{}{}", malloc_code, assign);
}
fn genassignment_old(value: &Statement, pos: usize, vars: &Vec<Variable>, bytecode: &mut ByteArray) -> String
{
    // println!("value: {}", value.to_string());
    println!("genassignment_old('{}')", value.to_string());
    let size = utils::get_type_size(value.datatype.clone());
    let expression = utils::parsebinary(value.clone(), &vars, bytecode);
    if value.datatype.datatype == StatementDatatype::String
    {
        bytecode.add_move_lit_to_reg(&pos.to_string(), Register::RBP, SizeType::QWORD);  
        return format!("movq -{}(%rbp), %rax\n{}", pos, expression);
    }
    else
    {
        bytecode.add_move_lit_from_reg_to_reg(&pos.to_string(), Register::RBP, Register::RBX, SizeType::QWORD);
        bytecode.add_move_reg_to_mem(Register::RAX, "0", Register::RBX, SizeType::QWORD); // TODO: size
        if size == 1
        {
            return format!("{}movq -{}(%rbp), %rbx\nmovb %al, (%rbx)\n", expression, pos);
        }
        else
        if size == 2
        {
            return format!("{}movq -{}(%rbp), %rbx\nmovw %ax, (%rbx)\n", expression, pos);
        }
        else
        if size == 4
        {
            return format!("{}movq -{}(%rbp), %rbx\nmovl %eax, (%rbx)\n", expression, pos);
        }
        else
        if size == 8
        {
            return format!("{}movq -{}(%rbp), %rbx\nmovq %rax, (%rbx)\n", expression, pos);
        }
    }
    panic!("Invalid size for assignment ({} bytes)", size);
}