use super::super::Statement;
pub fn gencall(statement: Statement) -> String
{
    let name = statement.name;
    let mut string = String::new();
    // save registers to stack (push)
    // call function
    // restore registers from stack (pop)
    string.push_str(&format!("push %rcx\npush %rdx\n"));
    string.push_str(&format!("call {}\n", name));
    string.push_str(&format!("pop %rdx\npop %rcx\n"));
    return string;
}