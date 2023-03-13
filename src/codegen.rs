mod versions;
pub use versions::OS;

pub mod linux;
use linux::Variable;

use crate::syntactic_analyser::statement::{StatementType, StatementDatatype};

use super::syntactic_analyser::statement::Statement;
pub struct Codegen
{
    statements: Vec<Statement>,
    data: String,
    os: OS,
}
impl Codegen
{
    pub fn new(statements: Vec<Statement>, os: OS) -> Codegen
    {
        Codegen { statements: statements, data: "".to_string(), os }
    }
    pub fn generate(&mut self)
    {
        if self.os == OS::Linux
        {
            self.generate_linux();
        }
        else
        {
            panic!("OS not supported");
        }
    }
    fn generate_linux(&mut self)
    {
        self.data.push_str(".data\n");
        self.data.push_str(".extern exit\n");
        self.data.push_str(".extern malloc\n");
        self.data.push_str(".extern free\n");
        self.data.push_str(".text\n");
        self.data.push_str(".globl _start\n");
        self.data.push_str("_start:\n");
        self.data.push_str("call main\n");
        self.data.push_str("movq %rax, %rdi\n");
        self.data.push_str("movq $60, %rax\n");
        self.data.push_str("syscall\n\n");
        self.data.push_str("");
        for statement in self.statements.clone()
        {
            let state = statement.clone();
            let func = linux::generate(state);
            self.data.push_str(func.as_str());
        }
        let registers = linux::utils::get_registers();
        for register in registers
        {
            self.data = self.data.replace(format!("push %{}\npop %{}\n", register, register).as_str(), "");
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