use bytecode::ByteArray;

use crate::codegen::bytecode;
use bytecode::{
    Register,
    SizeType,
};

use super::{
    Statement,
    Variable,
    utils,
    StatementType,
    generate_body,
    super::StatementDatatype,
};
pub fn genif(statement: Statement, vars: &Vec<Variable>, used_positions: &Vec<usize>, highest_position: &usize, if_points: &mut usize, bytecode: &mut ByteArray)
{
    *if_points += 1;
    if statement.statements.len() == 0
    {
        panic!("No value for if");
    }
    let condition = statement.statements[0].clone();
    if condition.datatype.datatype != StatementDatatype::Bool
    {
        panic!("Only booleans are supported as arguments for ifs");
    }
    let if_branch = statement.statements[1].clone();
    let else_branch = statement.statements[2].clone();
    // println!("if_branch: {}", if_branch.type_.to_string());
    if if_branch.type_ == else_branch.type_ && if_branch.type_ != StatementType::Body
    {
        panic!("If and else branches must be statements");
    }
    let mut condition_bytecode = ByteArray::new();
    let mut if_branch_bytecode = ByteArray::new();
    let mut else_branch_bytecode = ByteArray::new();
    utils::parsebinary(condition, vars, &mut condition_bytecode);
    let points = if_points.clone();
    generate_body(if_branch.statements, vars.clone(), used_positions.clone(), highest_position.clone(), if_points, &mut if_branch_bytecode);
    generate_body(else_branch.statements, vars.clone(), used_positions.clone(), highest_position.clone(), if_points, &mut else_branch_bytecode);
    bytecode.add_array(&condition_bytecode);
    bytecode.add_cmp_let("0".to_string(), Register::RAX, SizeType::BYTE); // 1 = true
    bytecode.add_jmp_if_eq(&format!(".Lelse{}", points));
    bytecode.add_array(&if_branch_bytecode);
    bytecode.add_jmp(&format!(".Lend{}", points));
    bytecode.add_entry(&format!(".Lelse{}", points));
    bytecode.add_array(&else_branch_bytecode);
    bytecode.add_entry(&format!(".Lend{}", points));
}