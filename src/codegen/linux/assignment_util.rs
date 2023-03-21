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
pub fn genassignment(statement: Statement, vars: &mut Vec<Variable>, mut used_positions: &mut Vec<usize>, mut highest_position: &mut usize, bytecode: &mut ByteArray)
{
    // println!("genassignment({})", statement.to_string());
    let var = statement.clone();
    let name = var.name.clone();
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
        if var.datatype.array_bounds.len() > 0
        {
            let mut array_size = 1;
            for bound in var.datatype.array_bounds.clone()
            {
                array_size *= bound;
            }
            genassignment_new(size * array_size, &value, pos, &vars, array_size, bytecode);
        }
        else
        {
            genassignment_new(size, &value, pos, &vars, 1, bytecode);
        }
    }
    else
    {
        genassignment_old(&value, pos, &vars, bytecode);
    }
}
fn genassignment_new(size: i32, value: &Statement, pos: usize, vars: &Vec<Variable>, size_multiplier: i32, bytecode: &mut ByteArray)
{
    let mut size = size;
    if value.datatype.datatype == StatementDatatype::String
    {
        let expression = value.name.clone();
        size = expression.len() as i32 - 2;
    }
    let size = size * size_multiplier;
    bytecode.add_comment("Allocating memory for variable");
    bytecode.add_move_lit_to_reg(&size.to_string(), Register::RDI, SizeType::QWORD);
    bytecode.add_call("malloc");
    // bytecode.add_move_lit_to_reg("8", Register::RBX, SizeType::QWORD);
    bytecode.add_sub_lit_reg("8", Register::RSP, SizeType::QWORD);
    bytecode.add_move_reg_to_mem(Register::RAX, &(pos as i32*-1).to_string(), Register::RBP, SizeType::QWORD);
    genassignment_old(value, pos, &vars, bytecode);
}
fn genassignment_old(value: &Statement, pos: usize, vars: &Vec<Variable>, bytecode: &mut ByteArray)
{
    // println!("value: {}", value.to_string());
    // println!("genassignment_old('{}')", value.to_string());
    let mut expression_bytecode = ByteArray::new();
    utils::parsebinary(value.clone(), &vars, &mut expression_bytecode);
    bytecode.add_comment("assign value to Variable");
    if value.datatype.datatype == StatementDatatype::String
    {
        bytecode.add_move_mem_to_reg(Register::RBP, &(pos as i32 * -1).to_string(), Register::RAX, SizeType::QWORD);
        bytecode.add_array(&expression_bytecode);
        return;
    }
    else
    {
        let size = SizeType::QWORD;
        bytecode.add_array(&expression_bytecode);
        bytecode.add_move_mem_to_reg(Register::RBP, &(pos as i32 * -1).to_string(), Register::RBX, SizeType::QWORD);
        bytecode.add_move_reg_to_mem(Register::RAX, "0", Register::RBX, size); // TODO: size
        return;
    }
}
pub fn genoverwrite_array(value: Statement, vars: &mut Vec<Variable>, bytecode: &mut ByteArray)
{
    let name = value.name.clone();
    let pos = utils::findvariableindex(&name, &vars);
    genload_array(&value, pos, &vars, bytecode);
    bytecode.add_push();
    let val = value.statements[0].clone();
    utils::parsebinary(val, &vars, bytecode);
    bytecode.add_pop(Register::RBX);
    bytecode.add_move_reg_to_mem(Register::RAX, "0", Register::RBX, SizeType::QWORD);
}
fn genload_array(value: &Statement, pos: usize, vars: &Vec<Variable>, bytecode: &mut ByteArray)
{
    println!("genload_array({})", value.to_string());
    let pos = pos as i32 * -1;
    bytecode.add_comment("Loading array");
    bytecode.add_move_mem_to_reg(Register::RBP, &pos.to_string(), Register::RBX, SizeType::QWORD);
    for i in 1..value.statements.len()
    {
        bytecode.add_push_reg(Register::RBX);
        let statement = value.statements[i].clone();
        utils::parsebinary(statement, &vars, bytecode);
        bytecode.add_pop(Register::RBX);
        bytecode.add_swap(Register::RBX, Register::RAX);
        bytecode.add_add(SizeType::QWORD);
        bytecode.add_swap(Register::RBX, Register::RAX);
    }
    bytecode.add_move_reg_to_reg(Register::RBX, Register::RAX, SizeType::QWORD);
}