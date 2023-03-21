mod versions;
pub use versions::OS;

pub mod linux;
pub mod bytecode;
pub use bytecode::ByteArray;

use crate::syntactic_analyser::statement::{
    StatementType,
    StatementDatatype,
    Statement,
    Datatype,
};
use super::syntactic_analyser::arguments::Arguments;

use std::collections::HashMap;
pub struct Codegen
{
    statements: Vec<Statement>,
    functions: HashMap<String, (Datatype, Arguments, Vec::<Statement>)>,
    data: String,
    os: OS,
    std_functions: HashMap<String, usize>,
    comments: bool,
}
impl Codegen
{
    pub fn new(statements: Vec<Statement>, functions: HashMap<String, (Datatype, Arguments, Vec::<Statement>)>,os: OS, std_functions: HashMap<String, usize>
        ,comments: bool) -> Codegen
    {
        Codegen { statements: statements, functions, data: "".to_string(), os , std_functions, comments}
    }
    pub fn generate(&mut self)
    {
        let mut bytecode = ByteArray::new();
        linux::generate(self.statements.clone(), self.functions.clone(), &mut bytecode);
        for std_function in self.std_functions.clone()
        {
            let name = std_function.0.clone();
            println!("{}: {}", name, std_function.1);
            if std_function.0.starts_with("std::print") && std_function.1 > 0
            {
                linux::basic_functions::generate_print(&name, &mut bytecode);
            }
        }
        let result = bytecode.generate(self.os, self.comments);
        self.data = result;
    }
    fn save_asm(&self)
    {
        linux::utils::save_assebly_code(&self.data, "out.s");
    }
    pub fn compile(&self, path: &str)
    {
        self.save_asm();
        if self.os == OS::Linux
        {
            linux::utils::compile_linux(path, !self.comments);
        }
        else
        {
            panic!("OS not supported");
        }
    }
}