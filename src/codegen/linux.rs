mod variable;
pub use variable::Variable;
pub mod assignment_util;
pub mod load_util;
pub mod return_util;
pub mod utils;
pub mod call_util;
pub fn generate(statement: super::Statement) -> String
{
    use super::StatementType;
    let mut vars = Vec::<Variable>::new();
    let mut used_positions = Vec::<usize>::new();
    let mut highest_position: usize = 0;
    let mut data = String::new();
    data.push_str(statement.name.as_str());
    data.push_str(":\n");
    // println!("statements: {}", statement.statements.len());
    data.push_str("push %rbp\n");
    data.push_str("push %rbx\n");
    data.push_str("push %rdi\n");
    data.push_str("push %rsi\n");
    data.push_str("mov %rsp, %rbp\n");
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
            let str = call_util::gencall(expr.clone());
            data.push_str(str.as_str());
        }
    }
    println!("vars: {}", vars.len());
    data.push_str("push %rax\n");
    for var in vars
    {
        data.push_str(format!("movq -{}(%rbp), %rdi\n", var.index.clone()*8).as_str());
        data.push_str("call free\n");
    }
    data.push_str("pop %rax\n");
    data.push_str("mov %rbp, %rsp\n");
    data.push_str("pop %rsi\n");
    data.push_str("pop %rdi\n");
    data.push_str("pop %rbx\n");
    data.push_str("pop %rbp\n");
    data.push_str("ret\n\n");
    data
}