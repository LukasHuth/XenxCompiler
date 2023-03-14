mod variable;
pub use variable::Variable;
pub mod assignment_util;
pub mod load_util;
pub mod return_util;
pub mod utils;
pub mod call_util;
pub mod basic_functions;
use super::{
    Arguments,
    Datatype,
    Statement,
};
use std::collections::HashMap;
pub fn generate(statements: Vec<Statement>, functions: HashMap<String, (Datatype, Arguments, Vec::<Statement>)>) -> String
{
    let mut data = String::new();
    data.push_str(".data\n");
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
    for statement in statements.clone()
    {
        let state = statement.clone();
        let functions = functions.clone();
        let name = statement.name.clone();
        let function = functions.get(&name);
        let args = function.unwrap().1.clone();
        let func = generate_function(state, args);
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

pub fn generate_function(statement: super::Statement, args: Arguments) -> String
{
    use super::StatementType;
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
    let mut highest_position: usize = argument_regs.len().clone();
    for i in 0..argument_regs.len()
    {
        if i < args.arguments.len()
        {
            let arg = args.arguments[i].clone();
            let name = arg.name.clone();
            let var = Variable::new(&name, i, true, arg.datatype.clone());
            vars.push(var);
        }
        data.push_str(format!("push %{}\n", argument_regs[i]).as_str());
        used_positions.push(i);
    }
    // data.push_str(print_first.as_str());
    for expr in statement.statements
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
    }
    // println!("vars: {}", vars.len());
    //*
    data.push_str("push %rax\n");
    for var in vars
    {
        if var.is_argument
        {
            continue;
        }
        let size = utils::get_type_size(var.datatype.clone());
        data.push_str(format!("movq -{}(%rbp), %rdi\n", var.index.clone()*8).as_str());
        data.push_str(&format!("movq ${}, %rsi\n",size));
        data.push_str("call free\n");
    }
    data.push_str("pop %rax\n");
    // */
    data.push_str("mov %rbp, %rsp\n");
    data.push_str("pop %rsi\n");
    data.push_str("pop %rdi\n");
    data.push_str("pop %rbx\n");
    data.push_str("pop %rbp\n");
    data.push_str("ret\n\n");
    data
}