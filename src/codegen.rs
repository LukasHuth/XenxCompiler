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
            let func = self.genfunction_linux(state);
            self.data.push_str(func.as_str());
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
    fn genfunction_linux(&mut self, statement: Statement) -> String
    {
        #[allow(unused_mut)]
        let mut vars = Vec::<Variable>::new();
        let mut used_positions = Vec::<usize>::new();
        let mut highest_position: usize = 0;
        let mut data = String::new();
        data.push_str(statement.name.as_str());
        data.push_str(":\n");
        // println!("statements: {}", statement.statements.len());
        data.push_str("push %rbp\n");
        data.push_str("mov %rsp, %rbp\n");
        for expr in statement.statements
        {
            // println!("|expr: {}", expr.to_string());
            // println!("|type: {}", expr.type_.to_string());
            if expr.type_ == StatementType::Variable
            {
                // println!("Assignment");
                let str = linux::assignment_util::genassignment(expr.clone(), &mut vars, &mut used_positions, &mut highest_position);
                data.push_str(str.as_str());
            }
            if expr.type_ == StatementType::Return
            {
                // println!("Return");
                // let str = self.genreturn(expr.clone());
                let str = linux::return_util::genreturn(expr.clone(), &mut vars);
                data.push_str(str.as_str());
                break;
            }
        }
        println!("vars: {}", vars.len());
        data.push_str("push %rax\n");
        for var in vars
        {
            data.push_str(format!("movq -{}(%rbp), %rdi\n", var.index.clone()*8).as_str());
            data.push_str("call free\n");
        }
        data.push_str("pop %rax\n");
        data.push_str("mov %rbp, %rsp\n");
        data.push_str("pop %rbp\n");
        data.push_str("ret\n\n");
        data
    }
}