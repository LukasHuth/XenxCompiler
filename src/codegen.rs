use std::collections::HashMap;

use crate::syntactic_analyser::statement::{StatementType, StatementDatatype};

use super::syntactic_analyser::statement::Statement;
pub struct Codegen
{
    statements: Vec<Statement>,
    data: String,
    var_space: VariableSpace,
    vars: HashMap<String, i32>,
}
pub struct VariableSpace
{
    int: i32,
    float: i32,
    string: i32,
    bool: i32,
    char: i32,
}
impl Codegen
{
    pub fn new(statements: Vec<Statement>, var_space: VariableSpace) -> Codegen
    {
        Codegen { statements: statements, data: "".to_string(), var_space, vars: HashMap::new() }
    }
    pub fn generate(&mut self)
    {
        self.data.push_str(".data\n");
        self.data.push_str(format!(".lcomm int, {}\n", self.var_space.int * 4).as_str());
        self.data.push_str(".text\n");
        self.data.push_str(".globl _start\n");
        self.data.push_str("_start:\n");
        self.data.push_str("call main\n");
        self.data.push_str("movl %eax, %edi\n");
        self.data.push_str("movl $60, %eax\n");
        // self.data.push_str("movl %ebx, %edi\n");
        self.data.push_str("syscall\n\n");
        self.data.push_str("");
        for statement in self.statements.clone()
        {
            let state = statement.clone();
            let func = self.genfunction(state);
            self.data.push_str(func.as_str());
        }
    }
    pub fn save_asm(&self)
    {
        use std::fs::File;
        use std::io::Write;
        let mut file = File::create("out.s").unwrap();
        file.write_all(self.data.as_bytes()).unwrap();
    }
    pub fn compile(&self, path: &str)
    {
        use std::process::Command;
        let mut command = Command::new("as");
        command.arg("-o");
        command.arg("out.o");
        command.arg("out.s");
        command.output().unwrap();
        match std::fs::remove_file("out.s")
        {
            Ok(_) => {},
            Err(_) => {panic!("Failed to remove out.s");},
        }
        let mut command = Command::new("ld");
        command.arg("-o");
        command.arg(path);
        command.arg("out.o");
        command.output().unwrap();
        match std::fs::remove_file("out.o")
        {
            Ok(_) => {},
            Err(_) => {panic!("Failed to remove out.o");},
        }
        println!("Compiled to {}", path);
    }
    fn genfunction(&mut self, statement: Statement) -> String
    {
        #[allow(unused_mut)]
        let mut vars = Vec::<String>::new();
        let mut data = String::new();
        data.push_str(statement.name.as_str());
        data.push_str(":\n");
        // println!("statements: {}", statement.statements.len());
        for expr in statement.statements
        {
            // println!("|expr: {}", expr.to_string());
            // println!("|type: {}", expr.type_.to_string());
            if expr.type_ == StatementType::Variable
            {
                // println!("Assignment");
                let str = self.genassignment(expr.clone());
                data.push_str(str.as_str());
            }
            if expr.type_ == StatementType::Return
            {
                // println!("Return");
                let str = self.genreturn(expr.clone());
                data.push_str(str.as_str());
            }
        }
        for var in vars.clone()
        {
            self.vars.remove(var.as_str());
        }
        data
    }
    fn genreturn(&mut self, statement: Statement) -> String
    {
        let var = statement.clone();
        let name = var.statements[0].clone();
        if name.type_ == StatementType::Literal
        {
            let value = name.name.clone();
            let value = value.parse::<i32>().unwrap();
            return format!("movl ${}, %eax\nret\n", value);
        }
        if name.type_ == StatementType::Variable
        {
            let name = name.name.clone();
            if !self.vars.contains_key(name.as_str())
            {
                panic!("Variable {} not found", name);
            }
            let pos = self.vars.get(name.as_str()).unwrap();
            return format!("movl {} + int, %eax\nret\n", pos);
        }
        return String::new();
    }
    fn genassignment(&mut self, statement: Statement) -> String
    {
        // println!("genassignment({})", statement.to_string());
        let var = statement.clone();
        let name = var.name.clone();
        if var.datatype.datatype != StatementDatatype::Int
        {
            panic!("Only int variables are supported for now");
        }
        if var.statements.len() == 0
        {
            panic!("No value for variable {}", name);
        }
        let value = var.statements[0].clone();
        let pos: i32;
        if self.vars.keys().any(|x| x == name.as_str())
        {
            pos = *self.vars.get(&name).unwrap();
        }
        else
        {
            pos = self.findempty(StatementDatatype::Int);
        }
        if value.type_ == StatementType::Literal
        {
            let value = value.name.clone();
            // println!("{} = {}", name, value);
            let value = value.parse::<i32>().unwrap();
            self.vars.insert(name, pos);
            return format!("movl ${}, {} + int\n", value, pos);
        }
        if value.type_ == StatementType::Call // TODO: till now only no args are supported
        {
            let value = value.name.clone();
            // println!("{} = {}", name, value);
            self.vars.insert(name, pos);
            return format!("call {}\nmovl %eax, {} + int\n", value, pos);
        }
        return format!("movl ${}, {} + int\n", 0, pos);
    }
    fn findempty(&self, datatype: StatementDatatype) -> i32
    {
        let offset = self.getoffset(datatype);
        let count = match datatype
        {
            StatementDatatype::Int => self.var_space.get_int(),
            StatementDatatype::Float => self.var_space.get_float(),
            StatementDatatype::String => self.var_space.get_string(),
            StatementDatatype::Bool => self.var_space.get_bool(),
            StatementDatatype::Char => self.var_space.get_char(),
            _ => panic!("Invalid datatype"),
        };
        let size = self.getsize(datatype);
        let mut poses = Vec::<i32>::new();
        for i in 0..count
        {
            poses.push(i * size + offset);
        }
        for key in self.vars.keys()
        {
            let pos = self.vars.get(key).unwrap();
            if poses.contains(pos)
            {
                let i = poses.iter().position(|&x| x == *pos).unwrap();
                poses.remove(i);
            }
        }
        let poses = poses.clone();
        if poses.len() == 0
        {
            panic!("No empty space for variable");
        }
        return poses[0];
    }
    fn getsize(&self, datatype: StatementDatatype) -> i32
    {
        match datatype
        {
            StatementDatatype::Int => 4,
            StatementDatatype::Float => 4,
            StatementDatatype::String => 4,
            StatementDatatype::Bool => 1,
            StatementDatatype::Char => 1,
            _ => panic!("Invalid datatype"),
        }
    }
    fn getoffset(&self, datatype: StatementDatatype) -> i32
    {
        let offset = 0;
        if datatype == StatementDatatype::Int
        {
            return offset;
        }
        let offset = offset + self.var_space.get_int() * 4;
        if datatype == StatementDatatype::Float
        {
            return offset;
        }
        let offset = offset + self.var_space.get_float() * 4;
        if datatype == StatementDatatype::String
        {
            return offset;
        }
        let offset = offset + self.var_space.get_string() * 4;
        if datatype == StatementDatatype::Bool
        {
            return offset;
        }
        let offset = offset + self.var_space.get_bool();
        if datatype == StatementDatatype::Char
        {
            return offset;
        }
        return 0;
    }
}
impl VariableSpace
{
    pub fn new() -> VariableSpace
    {
        VariableSpace { int: 0, float: 0, string: 0, bool: 0, char: 0 }
    }
    pub fn inc_int(&mut self)
    {
        self.int += 1;
    }
    pub fn inc_float(&mut self)
    {
        self.float += 1;
    }
    pub fn inc_string(&mut self)
    {
        self.string += 1;
    }
    pub fn inc_bool(&mut self)
    {
        self.bool += 1;
    }
    pub fn inc_char(&mut self)
    {
        self.char += 1;
    }
    pub fn get_int(&self) -> i32
    {
        self.int
    }
    pub fn get_float(&self) -> i32
    {
        self.float
    }
    pub fn get_string(&self) -> i32
    {
        self.string
    }
    pub fn get_bool(&self) -> i32
    {
        self.bool
    }
    pub fn get_char(&self) -> i32
    {
        self.char
    }
    pub fn dec_int(&mut self)
    {
        self.int -= 1;
    }
    pub fn dec_float(&mut self)
    {
        self.float -= 1;
    }
    pub fn dec_string(&mut self)
    {
        self.string -= 1;
    }
    pub fn dec_bool(&mut self)
    {
        self.bool -= 1;
    }
    pub fn dec_char(&mut self)
    {
        self.char -= 1;
    }
}