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
    temp_data: String,
    os: OS,
}
impl Codegen
{
    pub fn new(statements: Vec<Statement>, functions: HashMap<String, (Datatype, Arguments, Vec::<Statement>)>,os: OS) -> Codegen
    {
        Codegen { statements: statements, functions, data: "".to_string(), os, temp_data: String::new() }
    }
    pub fn generate(&mut self)
    {
        let mut bytecode = ByteArray::new();
        self.data = linux::generate(self.statements.clone(), self.functions.clone(), &mut bytecode);
        let result = bytecode.generate(self.os);
        self.temp_data = result;
    }
    fn save_asm(&self)
    {
        linux::utils::save_assebly_code(&self.data, "out.s");
        linux::utils::save_assebly_code(&self.temp_data, "test.s");
    }
    pub fn compile(&self, path: &str)
    {
        self.save_asm();
        if self.os == OS::Linux
        {
            linux::utils::compile_linux(path);
        }
        else
        {
            panic!("OS not supported");
        }
    }
}