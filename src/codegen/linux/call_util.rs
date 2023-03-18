use crate::codegen::bytecode;
use bytecode::ByteArray;
use bytecode::Register;

use super::{Variable, utils};
use super::super::{
    Statement,
    StatementDatatype
};
pub fn gencall(statement: Statement, vars: &Vec<Variable>, bytecode: &mut ByteArray)
{
    let name = statement.name;
    // save registers to stack (push)
    // call function
    // restore registers from stack (pop)
    let argc = statement.statements.len();
    let registers = [Register::RDI, Register::RSI, Register::RDX, Register::R10, Register::R8, Register::R9];
    // println!("argc: {}", argc);
    for i in 0..argc
    {
        // let i = argc - i - 1;
        let expr = statement.statements[i].clone();
        if expr.datatype.datatype != StatementDatatype::Int
        {
            panic!("Only integers are supported as arguments for now");
        }
        utils::parsebinary(expr, vars, bytecode);
        bytecode.add_push();
    }
    for i in 0..argc
    {
        let i = argc - i - 1;
        if i < registers.len()
        {
            bytecode.add_pop(registers[i]);
        }
    }
    bytecode.add_push_reg(Register::RCX);
    bytecode.add_push_reg(Register::RDX);
    bytecode.add_call(&name);
    bytecode.add_pop(Register::RDX);
    bytecode.add_pop(Register::RCX);
}