use super::super::Statement;
use super::Variable;
use super::super::StatementType;
use super::super::StatementDatatype;
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
        let expr = statement.statements[i].clone();
        if expr.datatype.datatype != StatementDatatype::Int
        {
            panic!("Only integers are supported as arguments for now");
        }
        let load_from: String;
        if expr.type_ == StatementType::Literal
        {
            load_from = format!("${}", expr.name.clone());
        }
        else
        if expr.type_ == StatementType::Variable
        {
            let load_var = super::load_util::load_variable(vars, expr.name.clone());
            // println!("load_var: {}", load_var);
            string.push_str(load_var.as_str());
            load_from = "%rax".to_string();
        }
        else
        {
            panic!("Invalid argument type");
        }

        if i < registers.len()
        {
            string.push_str(&format!("mov {}, {}\n", load_from, registers[i]));
        }
        else
        {
            string.push_str(&format!("push ${}\n", load_from));
        }
    }
    string.push_str(&format!("push %rcx\npush %rdx\n"));
    string.push_str(&format!("call {}\n", name));
    string.push_str(&format!("pop %rdx\npop %rcx\n"));
    return string;
}