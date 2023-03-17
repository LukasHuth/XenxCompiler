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
        ByteInstruction::Add =>
        {
            let rax_name = get_register_name(Register::RAX, size);
            let rbx_name = get_register_name(Register::RBX, size);
            data.push_str(format!("add %{}, %{}\n", rbx_name, rax_name).as_str());
        },
        ByteInstruction::Sub =>
        {
            let rax_name = get_register_name(Register::RAX, size);
            let rbx_name = get_register_name(Register::RBX, size);
            data.push_str(format!("sub %{}, %{}\n", rbx_name, rax_name).as_str());
        },
        ByteInstruction::Mul =>
        {
            let rax_name = get_register_name(Register::RAX, size);
            let rbx_name = get_register_name(Register::RBX, size);
            data.push_str(format!("imul %{}, %{}\n", rbx_name, rax_name).as_str());
        },
        ByteInstruction::Div =>
        {
            let rax_name = get_register_name(Register::RAX, size);
            let rbx_name = get_register_name(Register::RBX, size);
            data.push_str(format!("idiv %{}, %{}\n", rbx_name, rax_name).as_str());
        },
        _ => panic!("Instruction not supported"),
    }
    return data;
}
fn get_register_name(register: Register, size: SizeType) -> String
{
    let mut name = String::new();
    let is_named_register = is_named_register(register);
    match register
    {
        Register::RAX => name.push_str("ax"),
        Register::RBX => name.push_str("bx"),
        Register::RCX => name.push_str("cx"),
        Register::RDX => name.push_str("dx"),
        Register::RDI => name.push_str("di"),
        Register::RSI => name.push_str("si"),
        Register::RBP => name.push_str("bp"),
        Register::RSP => name.push_str("sp"),
        Register::R8 => name.push_str("r8"),
        Register::R9 => name.push_str("r9"),
        Register::R10 => name.push_str("r10"),
        Register::R11 => name.push_str("r11"),
        Register::R12 => name.push_str("r12"),
        Register::R13 => name.push_str("r13"),
        Register::R14 => name.push_str("r14"),
        Register::R15 => name.push_str("r15"),
    }
    if is_named_register
    {
        match size
        {
            SizeType::BYTE => name = name, // TODO: not implemented
            SizeType::WORD => name = name,
            SizeType::DWORD => name = "e".to_string() + name.as_str(),
            SizeType::QWORD => name = "r".to_string() + name.as_str(),
            SizeType::FLOAT => name = name, // TODO: not implemented
            SizeType::DOUBLE => name = name, // TODO: not implemented
            SizeType::CHAR => name = name, // TODO: not implemented
            SizeType::STRING => name = "r".to_string() + name.as_str(),
            SizeType::None => panic!("SizeType not supported"),
        }
    }
    else
    {
        match size
        {
            SizeType::BYTE => name.push_str("b"), // TODO: not implemented
            SizeType::WORD => name.push_str("w"), // TODO: not implemented
            SizeType::DWORD => name.push_str("d"),
            SizeType::QWORD => name.push_str(""),
            SizeType::FLOAT => name = name, // TODO: not implemented
            SizeType::DOUBLE => name = name, // TODO: not implemented
            SizeType::CHAR => name = name, // TODO: not implemented
            SizeType::STRING => name.push_str(""),
            SizeType::None => panic!("SizeType not supported"),
        }
    }
    return name;
}
fn is_named_register(register: Register) -> bool
{
    match register
    {
        Register::RAX => true,
        Register::RBX => true,
        Register::RCX => true,
        Register::RDX => true,
        Register::RDI => true,
        Register::RSI => true,
        Register::RBP => true,
        Register::RSP => true,
        Register::R8 => false,
        Register::R9 => false,
        Register::R10 => false,
        Register::R11 => false,
        Register::R12 => false,
        Register::R13 => false,
        Register::R14 => false,
        Register::R15 => false,
        _ => false,
    }
}