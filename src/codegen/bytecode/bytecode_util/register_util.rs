use super::{
    Register,
    SizeType,
};
pub fn get_name(register: Register, size: SizeType) -> String
{
    let mut name = String::new();
    let is_named_register = is_named_register(register);
    if size == SizeType::BYTE
    {
        if register == Register::RAX
        {
            return "al".to_string();
        }
        if register == Register::RBX
        {
            return "bl".to_string();
        }
        if register == Register::RCX
        {
            return "cl".to_string();
        }
        if register == Register::RDX
        {
            return "dl".to_string();
        }
    }
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
        Register::XMM0 => name.push_str("xmm0"),
        Register::XMM1 => name.push_str("xmm1"),
    }
    if is_named_register // TODO: implement float
    {
        match size
        {
            SizeType::BYTE => name = name, // just treat it as a word
            SizeType::WORD => name = name,
            SizeType::DWORD => name = "e".to_string() + name.as_str(),
            SizeType::QWORD => name = "r".to_string() + name.as_str(),
            SizeType::FLOAT => name = "r".to_string() + name.as_str(), // TODO: not implemented
            SizeType::DOUBLE => name = name, // TODO: not implemented
            SizeType::CHAR => name = name, // TODO: not implemented
            SizeType::STRING => name = "r".to_string() + name.as_str(),
            _ => name = name,
        }
    }
    else
    {
        match size
        {
            SizeType::BYTE => name.push_str("w"), // just treat it as a word
            SizeType::WORD => name.push_str("w"), // TODO: not implemented
            SizeType::DWORD => name.push_str("d"),
            SizeType::QWORD => name.push_str(""),
            SizeType::FLOAT => name = name, // TODO: not implemented
            SizeType::DOUBLE => name = name, // TODO: not implemented
            SizeType::CHAR => name = name, // TODO: not implemented
            SizeType::STRING => name.push_str(""),
            SizeType::None => name = name,
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
        _ => false,
    }
}
pub fn get_register_by_name(name: String) -> Register
{
    match name.as_str()
    {
        "rax" => Register::RAX,
        "rbx" => Register::RBX,
        "rcx" => Register::RCX,
        "rdx" => Register::RDX,
        "rdi" => Register::RDI,
        "rsi" => Register::RSI,
        "rbp" => Register::RBP,
        "rsp" => Register::RSP,
        "r8" => Register::R8,
        "r9" => Register::R9,
        "r10" => Register::R10,
        "r11" => Register::R11,
        "r12" => Register::R12,
        "r13" => Register::R13,
        "r14" => Register::R14,
        "r15" => Register::R15,
        _ => panic!("Register not supported"),
    }
}