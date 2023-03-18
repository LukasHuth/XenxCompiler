use crate::codegen::ByteArray;
use crate::codegen::bytecode::{Register, SizeType};
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
    // use this when the code is in intel syntax
    // let mut command = Command::new("nasm");
    // command.arg("-felf64");
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
pub fn save_assebly_code(str: &str, filename: &str) {
    use std::fs::File;
    use std::io::Write;
    let mut file = File::create(filename).unwrap();
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
    let size = 8;
    if used_positions.len() == 0
    {
        *highest_position=size;
        return *highest_position;
    }
    for i in 1..*highest_position+1
    {
        let mut found = false;
        for j in i..(i+size)
        {
            if used_positions.contains(&j)
            {
                found = true;
            }
        }
        if found
        {
            continue;
        }
        *highest_position+=size;
        return i;
    }
    *highest_position+=size;
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
pub fn parsebinary(statement: Statement, vars: &Vec<Variable>, bytecode: &mut ByteArray) -> String
{
    // TODO: bytecode
    let mut code = String::new();
    if statement.type_ == StatementType::Binary
    {
        let left = statement.statements[0].clone();
        let right = statement.statements[1].clone();
        let mut bytecode_left = ByteArray::new();
        let mut bytecode_right = ByteArray::new();
        let left = parsebinary(left, &vars, &mut bytecode_left);
        code.push_str(&left);
        bytecode.add_array(&bytecode_left);
        let right = parsebinary(right, &vars, &mut bytecode_right);
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
        if bytecode_left.is_same(&bytecode_right)
        {
            bytecode.add_move_reg_to_reg(Register::RAX, Register::RBX, SizeType::QWORD);
        }
        else
        {
            bytecode.add_push_reg(Register::RAX);
            bytecode.add_array(&bytecode_right);
        }
        if statement.name == "+"
        {
            code.push_str("addq %rbx, %rax\n");
            bytecode.add_add(SizeType::QWORD);
        }
        else
        if statement.name == "-"
        {
            code.push_str("subq %rax, %rbx\nmovq %rbx, %rax\n");
            bytecode.add_swap(Register::RAX, Register::RBX);
            bytecode.add_sub(SizeType::QWORD);
        }
        else
        if statement.name == "*"
        {
            code.push_str("imulq %rbx, %rax\n");
            bytecode.add_mul(SizeType::QWORD);
        }
        else
        if statement.name == "/"
        {
            code.push_str("movq %rax, %rcx\nmovq %rbx, %rax\nmovq %rcx, %rbx\nmovq $0, %rcx\ncqto\nidivq %rbx\n");
            bytecode.add_div(SizeType::QWORD);
        }
        else
        if statement.name == "=="
        {
            code.push_str("cmpq %rbx, %rax\nsete %al\nmovzbq %al, %rax\n");
            bytecode.add_move_reg_to_reg(Register::RAX, Register::RCX, SizeType::QWORD);
            bytecode.add_move_lit_to_reg("1", Register::RAX, SizeType::QWORD);
            bytecode.add_cmp_reg(Register::RBX, Register::RCX, SizeType::QWORD);
            bytecode.add_set_equal(Register::RAX);
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
        code = load_util::load_variable(&vars, statement.name.clone(), statement.datatype.clone(), bytecode);
    }
    else
    if statement.type_ == StatementType::Literal
    {
        if statement.datatype.datatype == StatementDatatype::Int
        {
            code = format!("movq ${}, %rax\n", statement.name);
            bytecode.add_move_lit_to_reg(&statement.name, Register::RAX, SizeType::QWORD);
        }
        else
        if statement.datatype.datatype == StatementDatatype::Bool
        {
            let val = if statement.name == "true" {1} else {0};
            code = format!("movb ${}, %al\n", val);
            bytecode.add_move_lit_to_reg(&statement.name, Register::RAX, SizeType::QWORD);
        }
        else
        if statement.datatype.datatype == StatementDatatype::Char
        {
            code = format!("movb $'{}', %al\n", statement.name);
            bytecode.add_move_lit_to_reg(&format!("'{}'", statement.name), Register::RAX, SizeType::BYTE);
        }
        else if statement.datatype.datatype == StatementDatatype::String
        {
            let mut str = statement.name.clone();
            str.remove(0);
            str.remove(str.len()-1);
            for i in 0..str.chars().count()
            {
                let char = str.chars().nth(i).unwrap();
                code.push_str(&format!("movb $'{}', {}(%rax)\n", char, i));
                bytecode.add_move_lit_to_reg(&format!("'{}'", statement.name), Register::RBX, SizeType::BYTE);
                bytecode.add_move_reg_to_mem(Register::RBX, &i.to_string(), Register::RAX, SizeType::BYTE);
            }
        }
        else
        {
            panic!("Invalid literal type");
        }
    }
    else if statement.type_ == StatementType::Unary
    {
        let left = statement.statements[0].clone();
        let mut left_bytecode = ByteArray::new();
        let left = parsebinary(left, &vars, &mut left_bytecode);
        code.push_str(&left);
        bytecode.add_array(&left_bytecode);
        if statement.name == "-"
        {
            code.push_str("negq %rax\n");
            bytecode.add_neg(Register::RAX);
        }
        else
        {
            panic!("Unary operator not supported (yet)");
        }
    }
    else if statement.type_ == StatementType::Call
    {
        let callstr = super::call_util::gencall(statement.clone(), &vars, bytecode);
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

pub fn move_literal_to_rax(state: Statement) -> String {
    let mut data = String::new();
    if state.type_ != StatementType::Literal {
        panic!("Statement is not a literal");
    }
    let literal = state.name.clone();
    if state.datatype.datatype == StatementDatatype::Int {
        data.push_str(&format!("movq ${}, %rax\n", literal));
    } else if state.datatype.datatype == StatementDatatype::Char {
        data.push_str(&format!("movb ${}, %al\n", literal));
    } else if state.datatype.datatype == StatementDatatype::Bool {
        let ret = if literal == "true" { 1 } else { 0 };
        data.push_str(&format!("movb ${}, %al\n", ret));
    } else if state.datatype.datatype == StatementDatatype::Float {
        data.push_str(&format!("movq ${}, %rax\n", literal));
    } else {
        panic!("Invalid literal type");
    }
    return data;
}