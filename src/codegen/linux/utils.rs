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
    command.output().unwrap();
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
use super::Variable;
pub fn findvariableindex(name: &str, variables: &Vec<Variable>) -> usize
{
    println!("Finding variable index for {}", name);
    println!("Variables: {:?}", variables);
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