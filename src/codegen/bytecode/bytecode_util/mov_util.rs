use super::{
    Instruction,
    register_util,
    SizeType,
};
pub fn mov_reg_to_mem(instruction: Instruction) -> String
{
    let register1 = instruction.get_register(1); // source
    let register2 = instruction.get_register(2); // destination
    let size = instruction.get_size_type();
    let arguments = instruction.get_arguments();
    if register1.is_none() || register2.is_none()
    {
        panic!("MovRegToMem expected 2 registers");
    }
    let register1 = register1.unwrap();
    let register2 = register2.unwrap();
    let source = register_util::get_name(register1.clone(), size);
    let destination = register_util::get_name(register2.clone(), SizeType::QWORD);
    let offset: String;
    if arguments.len() < 1
    {
        offset = "0".to_string();
    }
    else
    {
        offset = arguments[0].clone();
    }
    let offset = offset.parse::<i32>();
    if offset.is_err()
    {
        panic!("MovRegToMem expected offset to be an integer");
    }
    let offset = offset.unwrap();
    if offset < 0
    {
        return format!("mov [{} - {}], {}\n", destination, offset.abs(), source);
    }
    if offset == 0
    {
        return format!("mov [{}], {}\n", destination, source);
    }
    return format!("mov [{} + {}], {}\n", destination, offset, source);
}
pub fn mov_reg_to_reg(instruction: Instruction) -> String
{
    let register1 = instruction.get_register(1); // source
    let register2 = instruction.get_register(2); // destination
    let size = instruction.get_size_type();
    if register1.is_none() || register2.is_none()
    {
        panic!("Mov expected 2 registers");
    }
    // TODO: add support for mov reg, [reg]
    // TODO: add support for mov [reg], [reg]
    // TODO: add support for mov [reg], [reg + reg]
    let register1 = register1.unwrap();
    let register2 = register2.unwrap();
    let source = register_util::get_name(register1.clone(), size);
    let destination = register_util::get_name(register2.clone(), size);
    return format!("mov {}, {}\n", destination, source);
}
pub fn mov_mem_to_reg(instruction: Instruction) -> String
{
    let register1 = instruction.get_register(1); // source
    let register2 = instruction.get_register(2); // destination
    let size = instruction.get_size_type();
    let arguments = instruction.get_arguments();
    if register1.is_none() || register2.is_none()
    {
        panic!("MovMemToReg expected 2 registers");
    }
    let register1 = register1.unwrap();
    let register2 = register2.unwrap();
    let source = register_util::get_name(register1.clone(), SizeType::QWORD);
    let destination = register_util::get_name(register2.clone(), size);
    let offset: String;
    if arguments.len() < 1
    {
        offset = "0".to_string();
    }
    else
    {
        offset = arguments[0].clone();
    }
    let offset = offset.parse::<i32>();
    if offset.is_err()
    {
        panic!("MovMemToReg expected offset to be an integer");
    }
    let offset = offset.unwrap();
    if offset < 0
    {
        return format!("mov {}, [{} - {}]\n", destination, source, offset.abs());
    }
    if offset == 0
    {
        return format!("mov {}, [{}]\n", destination, source);
    }
    return format!("mov {}, [{} + {}]\n", destination, source, offset);
}
pub fn mov_lit_to_reg(instruction: Instruction) -> String
{
    let register1 = instruction.get_register(1); // destination
    let size = instruction.get_size_type();
    let arguments = instruction.get_arguments();
    if register1.is_none()
    {
        panic!("MovLitToReg expected 1 register");
    }
    if arguments.len() < 1
    {
        panic!("MovLitToReg expected 1 argument");
    }
    let register1 = register1.unwrap();
    let destination = register_util::get_name(register1.clone(), size);
    let argument = arguments[0].clone();
    if size == SizeType::FLOAT
    {
        return format!("mov {}, __float64__({})\n", destination, argument);
    }
    return format!("mov {}, {}\n", destination, argument);
}
pub fn mov_lit_to_mem(instruction: Instruction) -> String
{
    let register1 = instruction.get_register(1); // destination
    let size = instruction.get_size_type();
    let arguments = instruction.get_arguments();
    if register1.is_none()
    {
        panic!("MovLitToReg expected 1 register");
    }
    if arguments.len() < 1
    {
        panic!("MovLitToReg expected 1 argument");
    }
    let register1 = register1.unwrap();
    let destination = register_util::get_name(register1.clone(), SizeType::QWORD);
    let offset: String;
    let value: String;
    if arguments.len() < 1
    {
        offset = "0".to_string();
    }
    else
    {
        offset = arguments[0].clone();
    }
    if arguments.len() < 2
    {
        value = "0".to_string();
    }
    else
    if size == SizeType::FLOAT
    {
        value = format!("__float64__({})", arguments[1].clone());
    }
    else
    {
        value = arguments[1].clone();
    }
    let offset = offset.parse::<i32>();
    if offset.is_err()
    {
        panic!("MovRegToMem expected offset to be an integer");
    }
    let offset = offset.unwrap();
    // println!("{} {} {}", destination, offset, value);
    let size_name = size.get_name();
    if offset < 0
    {
        return format!("mov {} [{} - {}], {}\n", size_name, destination, offset.abs(), value);
    }
    if offset == 0
    {
        return format!("mov {} [{}], {}\n", size_name, destination, value);
    }
    return format!("mov {} [{} + {}], {}\n", size_name, destination, offset, value);
}