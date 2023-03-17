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
};
use super::bytecode;
use bytecode::{
    Register,
    SizeType,
};
use std::collections::HashMap;
pub fn generate(statements: Vec<Statement>, functions: HashMap<String, (Datatype, Arguments, Vec::<Statement>)>, bytecode: &mut ByteArray) -> String
{
    let mut data = String::new();
    // data.push_str(".data\n");
    // data.push_str(".extern exit\n");
    data.push_str(".extern printf\n");
    data.push_str(".data\n");
    data.push_str("format: .asciz \"%d\\n\"\n");
    data.push_str(".text\n");
    data.push_str(".globl _start\n");
    data.push_str("_start:\n");
    data.push_str("pop %rdi\n");
    data.push_str("movq %rsp, %rsi\n");
    // data.push_str("lea 0(%rsp), %rsi\n");
    data.push_str("call main\n");
    data.push_str("movq %rax, %rdi\n");
    data.push_str("call exit\n\n");
    data.push_str("");
    bytecode.add_section("data");
    // TODO: store constants (for printf)
    bytecode.add_section("text");
    bytecode.add_store_constant_string("format", "\"%d\\n\"");
    bytecode.add_section("text");
    bytecode.add_global("_start");
    bytecode.add_entry("_start");
    // if linux:
    bytecode.add_pop(Register::RDI, SizeType::QWORD);
    bytecode.add_move(Register::RSP, Register::RSI, SizeType::QWORD);
    bytecode.add_call("main");
    bytecode.add_move(Register::RAX, Register::RDI, SizeType::QWORD);
    bytecode.add_call("exit");
    let mut if_positions = 0;
    for statement in statements.clone()
    {
        let state = statement.clone();
        let functions = functions.clone();
        let name = statement.name.clone();
        let function = functions.get(&name);
        let args = function.unwrap().1.clone();
        let func = generate_function(state, args, &mut if_positions);
        data.push_str(func.as_str());
    }
    let registers = utils::get_registers();
    for register in registers
    {
        data = data.replace(format!("push %{}\npop %{}\n", register, register).as_str(), "");
    }
    let mut own_functions = String::from("# here begins the section for system functions\n\n");
    own_functions.push_str(&basic_functions::generate_malloc());
    own_functions.push_str(&basic_functions::generate_free());
    data.push_str(own_functions.as_str());
    return data;
}

pub fn generate_function(statement: super::Statement, args: Arguments, if_positions: &mut usize) -> String
{
    let mut vars = Vec::<Variable>::new();
    let mut used_positions = Vec::<usize>::new();
    let mut data = String::new();
    data.push_str(statement.name.as_str());
    data.push_str(":\n");
    // println!("statements: {}", statement.statements.len());
    data.push_str("push %rbp\n");
    data.push_str("push %rbx\n");
    data.push_str("push %rdi\n");
    data.push_str("push %rsi\n");
    data.push_str("mov %rsp, %rbp\n");
    let argument_regs = utils::get_argument_registers();
    let highest_position: usize = argument_regs.len().clone()*8; // 8 bytes per register
    for i in 0..argument_regs.len()
    {
        if i < args.arguments.len()
        {
            let arg = args.arguments[i].clone();
            let name = arg.name.clone();
            let var = Variable::new(&name, (i+1)*8, true, arg.datatype.clone());
            vars.push(var);
        }
        data.push_str(format!("push %{}\n", argument_regs[i]).as_str());
        for j in (i*8)..((i+1)*8)
        {
            used_positions.push(j);
        }
    }
    // */
    let body = generate_body(statement.statements, vars, used_positions, highest_position, if_positions);
    data.push_str(body.as_str());
    data.push_str("mov %rbp, %rsp\n");
    data.push_str("pop %rsi\n");
    data.push_str("pop %rdi\n");
    data.push_str("pop %rbx\n");
    data.push_str("pop %rbp\n");
    data.push_str("ret\n\n");
    data
}
pub fn generate_body(statements: Vec<Statement>, vars: Vec<Variable>, used_positions: Vec<usize>, highest_position: usize, if_points: &mut usize) -> String
{
    let old_vars = vars.clone();
    let mut vars = vars.clone();
    let mut used_positions = used_positions;
    let mut highest_position = highest_position;
    let mut data = String::new();
    // data.push_str(print_first.as_str());
    for expr in statements
    {
        // println!("|expr: {}", expr.to_string());
        // println!("|type: {}", expr.type_.to_string());
        if expr.type_ == StatementType::Variable
        {
            // println!("Assignment");
            let str = assignment_util::genassignment(expr.clone(), &mut vars, &mut used_positions, &mut highest_position);
            data.push_str(str.as_str());
        }
        if expr.type_ == StatementType::Return
        {
            // println!("Return");
            // let str = self.genreturn(expr.clone());
            let str = return_util::genreturn(expr.clone(), &mut vars);
            data.push_str(str.as_str());
            break;
        }
        if expr.type_ == StatementType::Call
        {
            let str = call_util::gencall(expr.clone(), &vars);
            println!("Unnecessary call: {}", expr.name);
            data.push_str(str.as_str());
        }
        if expr.type_ == StatementType::If
        {
            let str = if_util::genif(expr.clone(), &vars, &used_positions, &highest_position, if_points);
            data.push_str(str.as_str());
        }
    }
    // println!("vars: {}", vars.len());
    //*
    data.push_str("push %rax\n");
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
        data.push_str(format!("movq -{}(%rbp), %rdi\n", var.index.clone()).as_str());
        data.push_str(&format!("movq ${}, %rsi\n",size));
        data.push_str("call free\n");
    }
    data.push_str("pop %rax\n");
    return data;
}