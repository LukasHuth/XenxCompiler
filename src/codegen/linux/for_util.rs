use crate::{syntactic_analyser::statement::StatementType, codegen::bytecode::{Register, SizeType}};

use super::{
    Statement,
    Variable,
    ByteArray,
};

#[allow(unused_variables)]
pub fn genfor(expr: Statement, vars: &mut Vec<Variable>, used_positions: &mut Vec<usize>, highest_position: &mut usize, bytecode: &mut ByteArray,
              for_count: &mut usize, if_points: &mut usize)
{
    let head_expr = expr.statements.get(0);
    if head_expr.is_none()
    {
        panic!("head is none!");
    }
    let head_expr = head_expr.unwrap();
    let mut head_expr = head_expr.statements.clone();
    let mut body = expr.statements.clone();
    body.remove(0);
    let mut start_expr = Vec::<Statement>::new();
    while is_variable(&mut head_expr)
    {
        let start_expression = head_expr.get(0);
        if start_expression.is_none()
        {
            panic!("Start expression is none");
        }
        let start_expression = start_expression.unwrap().clone();
        start_expr.push(start_expression);
        head_expr.remove(0);
    }
    if !is_binary_operation(&mut head_expr)
    {
        panic!("Next Expression is not a binary Expression");
    }
    let binary_expr = head_expr.get(0).unwrap().clone();
    head_expr.remove(0);
    for start_expr_element in start_expr
    {
        super::assignment_util::genassignment(start_expr_element, vars, used_positions, highest_position, bytecode);
    }
    *for_count+=1;
    bytecode.add_entry(&format!("for_begin{}", for_count));
    super::utils::parsebinary(binary_expr, vars, bytecode);
    bytecode.add_cmp_reg(Register::RCX,Register::RBX, SizeType::BYTE);
    bytecode.add_jmp_if_eq(&format!("for_end{}", for_count));
    let immut_vars = vars.clone();
    let immut_used_positions = used_positions.clone();
    let immut_highest_position = highest_position.clone();
    super::generate_body(body, immut_vars.clone(), immut_used_positions.clone(), immut_highest_position.clone(), if_points,for_count, bytecode);
    super::generate_body(head_expr, immut_vars.clone(), immut_used_positions.clone(), immut_highest_position.clone(), if_points, for_count, bytecode);
    bytecode.add_jmp(&format!("for_begin{}", for_count));
    bytecode.add_entry(&format!("for_end{}", for_count))
}
fn is_variable(body: &mut Vec<Statement>) -> bool
{
    let expr = body.get(0);
    if expr.is_none()
    {
        return false;
    }
    let expr = expr.unwrap();
    if expr.type_ == StatementType::Variable || expr.type_ == StatementType::ArrayOverwrite || expr.type_ == StatementType::Assignment
    {
        return true;
    }
    return false;
}
fn is_binary_operation(body: &mut Vec<Statement>) -> bool
{
    let expr = body.get(0);
    if expr.is_none()
    {
        return false;
    }
    let expr = expr.unwrap();
    if expr.type_ == StatementType::Binary
    {
        return true;
    }
    return false;
}
