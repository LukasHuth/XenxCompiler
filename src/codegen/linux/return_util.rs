use bytecode::ByteArray;

use crate::codegen::bytecode;

use super::super::{
    Statement,
};
use super::{
    Variable,
    utils,
};

pub fn genreturn(statement: Statement, vars: &mut Vec<Variable>, bytecode: &mut ByteArray) -> String
{
    if statement.statements.len() == 0
    {
        panic!("No value for return");
    }
    let value = statement.statements[0].clone();
    return utils::parsebinary(value, vars, bytecode);
}