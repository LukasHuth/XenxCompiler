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
            | ByteInstruction::Mod | ByteInstruction::Not =>
        {
            let destination = get_register_name(Register::RAX, size);
            let source = get_register_name(Register::RBX, size);
            let operand = match inst
            {
                ByteInstruction::Add => "add",
                ByteInstruction::Sub => "sub",
                ByteInstruction::Mul => "mul",
                ByteInstruction::Div => "div",
                ByteInstruction::Or => "or",
                ByteInstruction::Xor => "xor",
                ByteInstruction::And => "and",
                ByteInstruction::Mod => "mod",
                ByteInstruction::Not => "not",
                _ => panic!("Invalid instruction"),
            };
            data.push_str(format!("{} {}, {}\n", operand, source, destination).as_str());
        },
        ByteInstruction::Jz | ByteInstruction::Jnz | ByteInstruction::Je | ByteInstruction::Jne | ByteInstruction::Jg | ByteInstruction::Jge | ByteInstruction::Jl
            | ByteInstruction::Jle | ByteInstruction::Jn | ByteInstruction::Jmp =>
        {
            let operation = match inst
            {
                ByteInstruction::Jz => "jz",
                ByteInstruction::Jnz => "jnz",
                ByteInstruction::Je => "je",
                ByteInstruction::Jne => "jne",
                ByteInstruction::Jg => "jg",
                ByteInstruction::Jge => "jge",
                ByteInstruction::Jl => "jl",
                ByteInstruction::Jle => "jle",
                ByteInstruction::Jn => "jn",
                ByteInstruction::Jmp => "jmp",
                _ => panic!("Invalid instruction"),
            };
            if arguments.len() < 1
            {
                panic!("{} expected a label", operation);
            }
            data.push_str(format!("{} {}\n", operation, arguments[0]).as_str());
        },
        ByteInstruction::LoadVariable =>
        {
            todo!();
        },
        ByteInstruction::StoreVariable =>
        {
            todo!();
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
        ByteInstruction::StoreValue =>
        {
            todo!();
        },
        ByteInstruction::LoadValue =>
        {
            todo!();
        },
        ByteInstruction::Shl | ByteInstruction::Shr =>
        {
            let operation = match inst
            {
                ByteInstruction::Shl => "shl",
                ByteInstruction::Shr => "shr",
                _ => panic!("Invalid instruction"),
            };
            if register1.is_none()
            {
                panic!("Shl/Shr expected a source");
            }
            if register2.is_none() && arguments.len() < 1
            {
                panic!("Shl/Shr expected a destination");
            }
            let register1 = register1.unwrap();
            let source = get_register_name(register1.clone(), size);
            let destination: String;
            if register2.is_none()
            {
                destination = arguments[0].clone();
            }
            else
            {
                let register2 = register2.unwrap();
                destination = get_register_name(register2.clone(), size);
            }
            data.push_str(format!("{} {}, {}\n", operation, source, destination).as_str());
        },
        ByteInstruction::Cmp =>
        {
            if register1.is_none() && arguments.len() < 1
            {
                panic!("Shl/Shr expected a source");
            }
            if register2.is_none() && arguments.len() < 1
            {
                panic!("Shl/Shr expected a destination");
            }
            let source: String;
            let destination: String;
            if register1.is_none()
            {
                source = arguments[0].clone();
            }
            else
            {
                let register1 = register1.unwrap();
                source = get_register_name(register1.clone(), size);
            }
            if register2.is_none()
            {
                destination = arguments[0].clone();
            }
            else
            {
                let register2 = register2.unwrap();
                destination = get_register_name(register2.clone(), size);
            }
            data.push_str(format!("cmp {}, {}\n", source, destination).as_str());
        },
        ByteInstruction::Mov =>
        {
            if register1.is_none() || register2.is_none()
            {
                panic!("Mov expected 2 registers");
            }
            // TODO: add support for mov [reg], reg
            // TODO: add support for mov reg, [reg]
            // TODO: add support for mov [reg], [reg]
            // TODO: add support for mov [reg], [reg + reg]
            let register1 = register1.unwrap();
            let register2 = register2.unwrap();
            let source = get_register_name(register1.clone(), size);
            let destination = get_register_name(register2.clone(), size);
            data.push_str(format!("mov {}, {}\n", destination, source).as_str());
        },
        ByteInstruction::Push=>
        {
            if register1.is_none() && arguments.len() < 1
            {
                panic!("Push expected 1 register or 1 argument");
            }
            let value: String;
            if register1.is_none()
            {
                value = arguments[0].clone();
            }
            else
            {
                let register1 = register1.unwrap();
                value = get_register_name(register1.clone(), size);
            }
            data.push_str(format!("push {}\n", value).as_str());
        },
        ByteInstruction::Pop =>
        {
            if register1.is_none()
            {
                panic!("Pop expected 1 register");
            }
            let register1 = register1.unwrap();
            let destination = get_register_name(register1.clone(), size);
            data.push_str(format!("pop {}\n", destination).as_str());
        },
        ByteInstruction::Ret =>
        {
            data.push_str("ret");
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