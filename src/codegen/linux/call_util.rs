use super::{Variable, utils};
use super::super::{
    Statement,
    StatementDatatype
};
pub fn gencall(statement: Statement, vars: &Vec<Variable>) -> String
{
    let name = statement.name;
    let mut string = String::new();
    // save registers to stack (push)
    // call function
    // restore registers from stack (pop)
    let argc = statement.statements.len();
    let registers = ["%rdi", "%rsi", "%rdx", "%rcx", "%r8", "%r9"];
    // println!("argc: {}", argc);
    for i in 0..argc
    {
        // let i = argc - i - 1;
        let expr = statement.statements[i].clone();
        if expr.datatype.datatype != StatementDatatype::Int
        {
            panic!("Only integers are supported as arguments for now");
        }
        let binary = utils::parsebinary(expr, vars);
        string.push_str(binary.as_str());
        string.push_str("push %rax\n");
    }
    for i in 0..argc
    {
        let i = argc - i - 1;
        if i < registers.len()
        {
            string.push_str(&format!("pop {}\n", registers[i]));
        }
    }
    string.push_str(&format!("push %rcx\npush %rdx\n"));
    string.push_str(&format!("call {}\n", name));
    string.push_str(&format!("pop %rdx\npop %rcx\n"));
    return string;
}