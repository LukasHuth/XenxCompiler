pub mod mov_util;
pub mod register_util;
pub mod xor_util;
pub mod movement_utils;
pub mod jump_util;
pub mod binary_util;
pub mod logical_util;
pub mod compare_util;
pub mod stack_util;
pub mod kernel_util;
use super::super::OS;
use super::{
    Instruction,
    ByteInstruction,
    Register,
    SizeType,
};
pub fn generate(instructions: Vec<Instruction>, os: OS) -> String
{
    let mut data = String::new();
    if os == OS::Linux
    {
        for instruction in instructions
        {
            data.push_str(generate_instruction(instruction, os).as_str());
        }
        return data;
    }
    else
    {
        panic!("OS not supported");
    }
}
fn generate_instruction(instruction: Instruction, os: OS) -> String
{
    if os == OS::Linux
    {
        return generate_instruction_linux(instruction);
    }
    else
    {
        panic!("OS not supported");
    }
}
// Done in intel syntax
fn generate_instruction_linux(instruction: Instruction) -> String
{
    let mut data = String::new();
    let arguments = instruction.get_arguments();
    let register1 = instruction.get_register(1);
    let register2 = instruction.get_register(2);
    let size = instruction.get_size_type();
    let inst = instruction.get_instruction();
    // TODO: add operand size
    match inst.clone()
    {
        ByteInstruction::Add | ByteInstruction::Sub | ByteInstruction::Mul | ByteInstruction::Div | ByteInstruction::Or | ByteInstruction::Xor | ByteInstruction::And
            | ByteInstruction::Not | ByteInstruction::Neg =>
        {
            let result = match inst
            {
                ByteInstruction::Add => binary_util::add(instruction),
                ByteInstruction::Sub => binary_util::sub(instruction),
                ByteInstruction::Mul => binary_util::mul(instruction),
                ByteInstruction::Div => binary_util::div(instruction),
                ByteInstruction::Or => logical_util::or(instruction),
                ByteInstruction::Xor => logical_util::xor(instruction),
                ByteInstruction::And => logical_util::and(instruction),
                ByteInstruction::Not => logical_util::not(instruction),
                ByteInstruction::Neg => logical_util::neg(instruction),
                _ => panic!("Invalid instruction"),
            };
            data.push_str(result.as_str());
        },
        ByteInstruction::Jz | ByteInstruction::Jnz | ByteInstruction::Je | ByteInstruction::Jne | ByteInstruction::Jg | ByteInstruction::Jge | ByteInstruction::Jl
            | ByteInstruction::Jle | ByteInstruction::Jn | ByteInstruction::Jmp =>
        {
            let result = match inst
            {
                ByteInstruction::Jz => jump_util::jump_zero(instruction),
                ByteInstruction::Jnz => jump_util::jump_not_zero(instruction),
                ByteInstruction::Je => jump_util::jump_equal(instruction),
                ByteInstruction::Jne => jump_util::jump_not_equal(instruction),
                ByteInstruction::Jg => jump_util::jump_greater(instruction),
                ByteInstruction::Jge => jump_util::jump_greater_equal(instruction),
                ByteInstruction::Jl => jump_util::jump_less(instruction),
                ByteInstruction::Jle => jump_util::jump_less_equal(instruction),
                ByteInstruction::Jn => jump_util::jump_negative(instruction),
                ByteInstruction::Jmp => jump_util::jump(instruction),
                _ => panic!("Invalid instruction"),
            };
            data.push_str(result.as_str());
        },
        ByteInstruction::Sete =>
        {
            let result = compare_util::set_equal(instruction);
            data.push_str(result.as_str());
        },
        ByteInstruction::LoadVariable =>
        {
            todo!();
        },
        ByteInstruction::StoreVariable => // register1 is the value, register2 is the destination
        {
            let result = mov_util::mov_reg_to_mem(instruction);
            data.push_str(result.as_str());
        },
        ByteInstruction::Swap =>
        {
            let result = movement_utils::swap(instruction);
            data.push_str(result.as_str());
        },
        ByteInstruction::LoadConstant =>
        {
            todo!();
        },
        ByteInstruction::LoadArgument =>
        {
            todo!();
        },
        ByteInstruction::StoreArgument =>
        {
            todo!();
        },
        ByteInstruction::StoreValue => // like mov reg, <value|reg>
        {
            if register2.is_none()
            {
                panic!("StoreValue expected a destination");
            }
            if register1.is_none() && arguments.len() < 1
            {
                panic!("StoreValue expected a source");
            }
            let register2 = register2.unwrap();
            let destination = register_util::get_name(register2.clone(), size);
            let source: String;
            if register1.is_none()
            {
                source = arguments[0].clone();
            }
            else
            {
                let register1 = register1.unwrap();
                source = register_util::get_name(register1.clone(), size);
            }
            data.push_str(format!("mov {}, {}\n", destination, source).as_str());
        },
        ByteInstruction::LoadValue => // unknown and unused at the moment
        {
            todo!();
        },
        ByteInstruction::Shl | ByteInstruction::Shr =>
        {
            let result = match inst
            {
                ByteInstruction::Shl => binary_util::shl(instruction),
                ByteInstruction::Shr => binary_util::shr(instruction),
                _ => panic!("Invalid instruction"),
            };
            data.push_str(result.as_str());
        },
        ByteInstruction::Syscall =>
        {
            let result = kernel_util::syscall();
            data.push_str(result.as_str());
        }
        ByteInstruction::Cmp =>
        {
            let result = compare_util::compare(instruction);
            data.push_str(result.as_str());
        },
        ByteInstruction::MovRegToReg =>
        {
            let result = mov_util::mov_reg_to_reg(instruction);
            data.push_str(result.as_str());
        },
        ByteInstruction::MovRegToMem =>
        {
            let result = mov_util::mov_reg_to_mem(instruction);
            data.push_str(result.as_str());
        },
        ByteInstruction::MovMemToReg =>
        {
            let result = mov_util::mov_mem_to_reg(instruction);
            data.push_str(result.as_str());
        },
        ByteInstruction::MovLitToReg =>
        {
            let result = mov_util::mov_lit_to_reg(instruction);
            data.push_str(result.as_str());
        },
        ByteInstruction::Push=>
        {
            let result = stack_util::push(instruction);
            data.push_str(result.as_str());
        },
        ByteInstruction::Pop =>
        {
            let result = stack_util::pop(instruction);
            data.push_str(result.as_str());
        },
        ByteInstruction::Ret =>
        {
            data.push_str("ret\n");
        },
        ByteInstruction::Call =>
        {
            if arguments.len() < 1
            {
                panic!("Call expected 1 argument");
            }
            let argument = arguments[0].clone();
            data.push_str(format!("call {}\n", argument).as_str());
        },
        ByteInstruction::Section =>
        {
            if arguments.len() < 1
            {
                panic!("Section expected 1 argument");
            }
            let argument = arguments[0].clone();
            data.push_str(format!("section .{}\n", argument).as_str());
        },
        ByteInstruction::Global =>
        {
            if arguments.len() < 1
            {
                panic!("Global expected 1 argument");
            }
            let argument = arguments[0].clone();
            data.push_str(format!("global {}\n", argument).as_str());
        },
        ByteInstruction::External =>
        {
            if arguments.len() < 1
            {
                panic!("Extern expected 1 argument");
            }
            let argument = arguments[0].clone();
            data.push_str(format!("extern {}\n", argument).as_str());
        },
        ByteInstruction::Entry =>
        {
            if arguments.len() < 1
            {
                panic!("Label expected 1 argument");
            }
            let argument = arguments[0].clone();
            data.push_str(format!("{}:\n", argument).as_str());
        },
        ByteInstruction::StoreConstant =>
        {
            if arguments.len() < 2
            {
                panic!("StoreConstant expected 2 argument");
            }
            let name = arguments[0].clone();
            let value = arguments[1].clone();
            let size_name = get_constant_size(size);
            if size == SizeType::STRING
            {
                data.push_str(format!("{}: {} '{}', 0\n", name, size_name, value).as_str());
            }
            else
            {
                data.push_str(format!("{}: {} {}\n", name, size_name, value).as_str());
            }
        },
        // _ => print!("")//panic!("Instruction not supported"),
    }
    return data;
}
fn get_constant_size(size: SizeType) -> String
{
    match size
    {
        SizeType::BYTE => return String::from("db"),
        SizeType::WORD => return String::from("dw"),
        SizeType::DWORD => return String::from("ddw"),
        SizeType::QWORD => return String::from("dq"),
        SizeType::STRING => return String::from("db"),
        _ => panic!("Size not supported (yet)"),
    }
}
fn get_register_names(instruction: Instruction) -> (String, String)
{
    let r1 = instruction.get_register(1);
    let r2 = instruction.get_register(2);
    if r1.is_none() || r2.is_none()
    {
        panic!("expected 2 registers");
    }
    let r1 = r1.unwrap();
    let r2 = r2.unwrap();
    let size = instruction.get_size_type();
    let r1 = register_util::get_name(r1, size);
    let r2 = register_util::get_name(r2, size);
    (r1, r2)
}
fn get_register_name(instruction: Instruction) -> String
{
    let r1 = instruction.get_register(1);
    if r1.is_none()
    {
        panic!("expected 1 register");
    }
    let r1 = r1.unwrap();
    let size = instruction.get_size_type();
    let r1 = register_util::get_name(r1, size);
    r1
}