use crate::codegen::bytecode;
use bytecode::ByteArray;
use bytecode::Register;

use super::{Variable, utils};
use super::super::{
    Statement,
    StatementDatatype
};
pub fn gencall(statement: Statement, vars: &Vec<Variable>, bytecode: &mut ByteArray) -> String
{
    let name = statement.name;
    let mut string = String::new();
    // save registers to stack (push)
    // call function
    // restore registers from stack (pop)
    let argc = statement.statements.len();
    let registers = ["%rdi", "%rsi", "%rdx", "%rcx", "%r8", "%r9"];
    let rregisters = [Register::RDI, Register::RSI, Register::RDX, Register::RCX, Register::R8, Register::R9];
    // println!("argc: {}", argc);
    for i in 0..argc
    {
        // let i = argc - i - 1;
        let expr = statement.statements[i].clone();
        if expr.datatype.datatype != StatementDatatype::Int
        {
            panic!("Only integers are supported as arguments for now");
        }
        let binary = utils::parsebinary(expr, vars, bytecode);
        string.push_str(binary.as_str());
        string.push_str("push %rax\n");
        bytecode.add_push();
    }
    for i in 0..argc
    {
        let i = argc - i - 1;
        if i < registers.len()
        {
            string.push_str(&format!("pop {}\n", registers[i]));
            bytecode.add_pop(rregisters[i]);
        }
    }
    string.push_str(&format!("push %rcx\npush %rdx\n"));
    bytecode.add_push_reg(Register::RCX);
    bytecode.add_push_reg(Register::RDX);
    string.push_str(&format!("call {}\n", name));
    bytecode.add_call(&name);
    string.push_str(&format!("pop %rdx\npop %rcx\n"));
    bytecode.add_pop(Register::RDX);
    bytecode.add_pop(Register::RCX);
    return string;
}