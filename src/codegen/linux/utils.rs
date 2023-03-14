use crate::syntactic_analyser::statement::Datatype;

use super::super::{
    Statement,
    StatementType,
    StatementDatatype,
};
use super::{
    Variable,
    load_util
};
pub fn compile_linux(path: &str) {
    let remove_files = false;
    use std::process::Command;
    let mut command = Command::new("as");
    command.arg("-o");
    command.arg("out.o");
    command.arg("out.s");
    command.output().unwrap();
    if remove_files
    {
        match std::fs::remove_file("out.s")
        {
            Ok(_) => {},
            Err(_) => {panic!("Failed to remove out.s");},
        }
    }
    let mut command = Command::new("ld");
    command.arg("-dynamic-linker");
    command.arg("/lib64/ld-linux-x86-64.so.2");
    command.arg("-o");
    command.arg(path);
    command.arg("out.o");
    command.arg("-lc");
    let res = command.output().unwrap();
    if res.status.code().unwrap() != 0
    {
        println!("{}", res.stderr.len());
        panic!("Failed to compile");
    }
    match std::fs::remove_file("out.o")
    {
        Ok(_) => {},
        Err(_) => {panic!("Failed to remove out.o");},
    }
    println!("Compiled to {}", path);
}
pub fn save_assebly_code(str: &str) {
    use std::fs::File;
    use std::io::Write;
    let mut file = File::create("out.s").unwrap();
    file.write_all(str.as_bytes()).unwrap();
}

pub fn findvariableindex(name: &str, variables: &Vec<Variable>) -> usize
{
    // println!("Finding variable index for {}", name);
    // println!("Variables: {:?}", variables);
    for var in variables
    {
        if var.name == name
        {
            return var.index;
        }
    }
    panic!("Variable {} not found", name);
}
pub fn havevariable(name: &str, variables: &Vec<Variable>) -> bool
{
    for var in variables
    {
        if var.name == name
        {
            return true;
        }
    }
    false
}
pub fn findemptyposition(used_positions: &mut Vec<usize>, highest_position: &mut usize) -> usize
{
    if used_positions.len() == 0
    {
        *highest_position=1;
        return *highest_position;
    }
    for i in 1..*highest_position+1
    {
        if used_positions.contains(&i)
        {
            continue;
        }
        return i;
    }
    *highest_position+=1;
    return *highest_position;
}
// source: https://www.tortall.net/projects/yasm/manual/html/arch-x86-registers.html
pub fn get_registers() -> Vec<String>
{
    let general_registers = get_general_register_names();
    let mut registers = vec![];
    for reg in general_registers
    {
        registers.push(reg.clone());
        registers.push(format!("e{}", reg));
        registers.push(format!("r{}", reg));
    }
    for i in 8..=15
    {
        registers.push(format!("r{}", i));
        registers.push(format!("r{}w", i));
        registers.push(format!("r{}b", i));
        registers.push(format!("r{}d", i));
    }
    return registers;
}
fn get_general_register_names() -> Vec<String>
{
    let registers = vec!["ax".to_string(),
    "bx".to_string(), "cx".to_string(), "dx".to_string(),
    "si".to_string(), "di".to_string(), "bp".to_string(),
    "sp".to_string()];
    return registers;
}
pub fn get_argument_registers() -> Vec<String>
{
    let registers = vec!["rdi".to_string(), "rsi".to_string(), "rdx".to_string(),
    "rcx".to_string(), "r8".to_string(), "r9".to_string()];
    return registers;
}
pub fn is_argument(name: &str, vars: &Vec<Variable>) -> bool
{
    for var in vars
    {
        if var.name == name
        {
            return var.is_argument;
        }
    }
    panic!("Variable {} not found", name);
}
pub fn parsebinary(statement: Statement, vars: &Vec<Variable>) -> String
{
    let mut code = String::new();
    if statement.type_ == StatementType::Binary
    {
        let left = statement.statements[0].clone();
        let right = statement.statements[1].clone();
        let left = parsebinary(left, &vars);
        code.push_str(&left);
        let right = parsebinary(right, &vars);
        if left == right
        {
            code.push_str("movq %rax, %rbx\n");
        }
        else
        {
            code.push_str("pushq %rax\n");
            code.push_str(&right);
            code.push_str("popq %rbx\n");
        }
        if statement.name == "+"
        {
            code.push_str("addq %rbx, %rax\n");
        }
        else
        if statement.name == "-"
        {
            code.push_str("subq %rax, %rbx\nmovq %rbx, %rax\n");
        }
        else
        if statement.name == "*"
        {
            code.push_str("imulq %rbx, %rax\n");
        }
        else
        if statement.name == "/"
        {
            code.push_str("movq %rax, %rcx\nmovq %rbx, %rax\nmovq %rcx, %rbx\nmovq $0, %rcx\ncqto\nidivq %rbx\n");
        }
        else
        {
            panic!("Invalid binary operator");
        }
        return code;
    }
    else
    if statement.type_ == StatementType::Variable
    {
        code = load_util::load_int_variable(&vars, statement.name.clone());
    }
    else
    if statement.type_ == StatementType::Literal
    {
        code = format!("movq ${}, %rax\n", statement.name);
    }
    else if statement.type_ == StatementType::Unary
    {
        let left = statement.statements[0].clone();
        let left = parsebinary(left, &vars);
        code.push_str(&left);
        if statement.name == "-"
        {
            code.push_str("negq %rax\n");
        }
        else
        {
            panic!("Unary operator not supported (yet)");
        }
    }
    else if statement.type_ == StatementType::Call
    {
        let callstr = super::call_util::gencall(statement.clone(), &vars);
        code.push_str(callstr.as_str());
    }
    else
    {
        println!("{:?}", statement);
        panic!("Invalid statement type");
    }
    return code;
}
pub fn get_type_size(datatype: Datatype) -> i32
{
    let mut size = match datatype.datatype
    {
        StatementDatatype::Int => 8,    // 8 bytes for int
        StatementDatatype::Char => 1,   // 1 byte for char
        StatementDatatype::String => 8, // 8 bytes for pointer
        StatementDatatype::Bool => 1,   // 1 byte for bool
        StatementDatatype::Void => 0,   // 0 bytes for void
        StatementDatatype::Float => 8,  // 8 bytes for double
    };
    for i in 0..datatype.array_bounds.len()
    {
        let bound = datatype.array_bounds[i].clone();
        size = size * bound;
    }
    return size;
}