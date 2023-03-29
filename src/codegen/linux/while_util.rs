use crate::codegen::bytecode::SizeType;

use super::{Statement, Variable, ByteArray, Register};
pub fn genwhile(expr: Statement, vars: &mut Vec<Variable>, used_positions: &mut Vec<usize>, highest_position: &mut usize, bytecode: &mut ByteArray,
              for_count: &mut usize, if_points: &mut usize)
{
    let bool_expr = expr.statements.get(0);
    if bool_expr.is_none()
    {
        panic!("bool expression of while is none");
    }
    let bool_expr = bool_expr.unwrap();
    let bool_expr = bool_expr.clone();
    let number = *for_count;
    bytecode.add_entry(&format!("while_start{}", number));
    super::utils::parsebinary(bool_expr, vars, bytecode);
    bytecode.add_cmp_let("0".to_string(), Register::RAX, SizeType::QWORD);
    *for_count +=1;
    bytecode.add_jmp_if_eq(&format!("while_end{}", number));
    let mut body = expr.statements.clone();
    body.remove(0);
    let immut_vars = vars.clone();
    let immut_used_position = used_positions.clone();
    let immut_highest_position = highest_position.clone();
    super::generate_body(body, immut_vars, immut_used_position, immut_highest_position, if_points, for_count, bytecode);
    bytecode.add_jmp(&format!("while_start{}", number));
    bytecode.add_entry(&format!("while_end{}", number));
}
