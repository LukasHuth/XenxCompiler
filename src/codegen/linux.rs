mod variable;
pub use variable::Variable;
pub mod assignment_util;
pub mod load_util;
pub mod return_util;
pub mod utils;
pub mod call_util;
pub mod basic_functions;
pub mod if_util;
use super::{
    Arguments,
    Datatype,
    Statement,
    StatementType,
    ByteArray,
    bytecode,
};
use bytecode::bytecode_util;
use bytecode_util::register_util;
use bytecode::{
    Register,
    SizeType,
};
use std::collections::HashMap;
pub fn generate(statements: Vec<Statement>, functions: HashMap<String, (Datatype, Arguments, Vec::<Statement>)>, bytecode: &mut ByteArray)
{
    bytecode.add_section("data");
    // TODO: store constants (for printf)
    bytecode.add_store_constant_string("format", "%d\\n");
    bytecode.add_section("text");
    bytecode.add_external("printf");
    bytecode.add_global("_start");
    bytecode.add_entry("_start");
    // if linux:
    bytecode.add_pop(Register::RDI);
    bytecode.add_move_reg_to_reg(Register::RSP, Register::RSI, SizeType::QWORD);
    bytecode.add_call("main");
    bytecode.add_move_reg_to_reg(Register::RAX, Register::RDI, SizeType::QWORD);
    bytecode.add_call("exit");
    bytecode.add_ret();
    let mut if_positions = 0;
    for statement in statements.clone()
    {
        let state = statement.clone();
        let functions = functions.clone();
        let name = statement.name.clone();
        let function = functions.get(&name);
        let args = function.unwrap().1.clone();
        generate_function(state, args, &mut if_positions, bytecode);
        // data.push_str(func.as_str());
    }
    basic_functions::generate_malloc(bytecode);
    basic_functions::generate_free(bytecode);
    basic_functions::generate_exit(bytecode);
    // return data;
}

pub fn generate_function(statement: super::Statement, args: Arguments, if_positions: &mut usize, bytecode: &mut ByteArray)
{
    let mut vars = Vec::<Variable>::new();
    let mut used_positions = Vec::<usize>::new();
    bytecode.add_entry(statement.name.as_str());
    bytecode.add_push_reg(Register::RBP);
    bytecode.add_push_reg(Register::RBX);
    bytecode.add_push_reg(Register::RDI);
    bytecode.add_push_reg(Register::RSI);
    bytecode.add_move_reg_to_reg(Register::RSP, Register::RBP, SizeType::QWORD);
    let argument_regs = utils::get_argument_registers();
    let highest_position: usize = args.arguments.len().clone()*8; // 8 bytes per register
    for i in 0..args.arguments.len()
    {
        let arg = args.arguments[i].clone();
        let name = arg.name.clone();
        let var = Variable::new(&name, (i+1)*8, true, arg.datatype.clone());
        vars.push(var);
        let register = register_util::get_register_by_name(argument_regs[i].clone());
        bytecode.add_push_reg(register);
        for j in (i*8)..((i+1)*8)
        {
            used_positions.push(j);
        }
    }
    // */
    generate_body(statement.statements, vars, used_positions, highest_position, if_positions, bytecode);
    bytecode.add_move_reg_to_reg(Register::RBP, Register::RSP, SizeType::QWORD);
    bytecode.add_pop(Register::RSI);
    bytecode.add_pop(Register::RDI);
    bytecode.add_pop(Register::RBX);
    bytecode.add_pop(Register::RBP);
    bytecode.add_ret();
}
pub fn generate_body(statements: Vec<Statement>, vars: Vec<Variable>, used_positions: Vec<usize>, highest_position: usize, if_points: &mut usize, bytecode: &mut ByteArray)
{
    let old_vars = vars.clone();
    let mut vars = vars.clone();
    let mut used_positions = used_positions;
    let mut highest_position = highest_position;
    for expr in statements
    {
        if expr.type_ == StatementType::Variable
        {
            assignment_util::genassignment(expr.clone(), &mut vars, &mut used_positions, &mut highest_position, bytecode);
        }
        if expr.type_ == StatementType::Return
        {
            return_util::genreturn(expr.clone(), &mut vars, bytecode);
            break;
        }
        if expr.type_ == StatementType::Call
        {
            call_util::gencall(expr.clone(), &vars, bytecode);
            println!("Unnecessary call: {}", expr.name);
        }
        if expr.type_ == StatementType::If
        {
            if_util::genif(expr.clone(), &vars, &used_positions, &highest_position, if_points, bytecode);
        }
    }
    bytecode.add_push();
    for i in old_vars.len()..vars.len()
    {
        let var = vars[i].clone();
        if var.is_argument
        {
            continue;
        }
        let mut size = utils::get_type_size(var.datatype.clone());
        if var.is_string
        {
            size = var.name.len()as i32-2;
        }
        let offset = var.index.clone().to_string();
        bytecode.add_move_mem_to_reg(Register::RBP, &offset, Register::RDI, SizeType::QWORD);
        bytecode.add_move_lit_to_reg(&size.to_string(), Register::RSI, SizeType::QWORD);
        bytecode.add_call("free");
    }
    bytecode.add_pop(Register::RAX);
}