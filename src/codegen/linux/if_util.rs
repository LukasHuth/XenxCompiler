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
    super::StatementDatatype,
};
pub fn genif(statement: Statement, vars: &Vec<Variable>, used_positions: &Vec<usize>, highest_position: &usize, if_points: &mut usize, bytecode: &mut ByteArray) -> String
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
    println!("if_branch: {}", if_branch.type_.to_string());
    if if_branch.type_ == else_branch.type_ && if_branch.type_ != StatementType::Body
    {
        panic!("If and else branches must be statements");
    }
    let mut string = String::new();
    let mut condition_bytecode = ByteArray::new();
    let mut if_branch_bytecode = ByteArray::new();
    let mut else_branch_bytecode = ByteArray::new();
    let load_value = utils::parsebinary(condition, vars, &mut condition_bytecode);
    let if_branch = super::generate_body(if_branch.statements, vars.clone(), used_positions.clone(), highest_position.clone(), if_points, &mut if_branch_bytecode);
    let else_branch = super::generate_body(else_branch.statements, vars.clone(), used_positions.clone(), highest_position.clone(), if_points, &mut else_branch_bytecode);
    string.push_str(&load_value);
    string.push_str(&format!("cmp $0, %al\nje .Lelse{}\n", if_points));
    string.push_str(&if_branch);
    string.push_str(&format!("jmp .Lend{}\n.Lelse{}:\n", if_points, if_points));
    string.push_str(&else_branch);
    string.push_str(&format!(".Lend{}:\n", if_points));
    bytecode.add_array(&condition_bytecode);
    bytecode.add_cmp_let("0".to_string(), Register::RAX, SizeType::BYTE);
    bytecode.add_jmp_if_eq(&format!(".Lelse{}", if_points));
    bytecode.add_array(&if_branch_bytecode);
    bytecode.add_jmp(&format!(".Lend{}", if_points));
    bytecode.add_entry(&format!(".Lelse{}", if_points));
    bytecode.add_array(&else_branch_bytecode);
    bytecode.add_entry(&format!(".Lend{}", if_points));
    return string;
}