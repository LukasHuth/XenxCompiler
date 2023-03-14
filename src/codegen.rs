mod versions;
pub use versions::OS;

pub mod linux;

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
}
impl Codegen
{
    pub fn new(statements: Vec<Statement>, functions: HashMap<String, (Datatype, Arguments, Vec::<Statement>)>,os: OS) -> Codegen
    {
        Codegen { statements: statements, functions, data: "".to_string(), os }
    }
    pub fn generate(&mut self)
    {
        if self.os == OS::Linux
        {
            self.data = linux::generate(self.statements.clone(), self.functions.clone());
        }
        else
        {
            panic!("OS not supported");
        }
    }
    fn save_asm(&self)
    {
        linux::utils::save_assebly_code(self.data.as_str());
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