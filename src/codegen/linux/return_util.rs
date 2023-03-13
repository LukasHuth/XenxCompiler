use super::super::Statement;
use super::Variable;
use super::super::StatementType;
use super::super::StatementDatatype;
use super::load_util;

pub fn genreturn(statement: Statement, vars: &mut Vec<Variable>) -> String
{
    if statement.statements.len() == 0
    {
        panic!("No value for return");
    }
    let value = statement.statements[0].clone();
    if value.type_ != StatementType::Literal && value.type_ != StatementType::Variable
    {
        panic!("Only literals and variables are supported for now");
    }
    if value.type_ == StatementType::Literal
    {
        if value.datatype.datatype != StatementDatatype::Int
        {
            panic!("Only int variables are supported for now");
        }
        let value = value.name.clone();
        return format!("movq ${}, %rax\n", value);
    }
    if value.type_ == StatementType::Variable
    {
        let value = value.name.clone();
        // println!("name: {}", value);
        let load_code = load_util::load_int_variable(&vars, value);
        return load_code;
    }
    return String::new();
}