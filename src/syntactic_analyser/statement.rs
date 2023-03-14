use std::fmt::{Formatter,Error,Display};
use super::{
    Arguments,
    util
};
use std::collections::HashMap;
#[derive(Clone, Debug)]
pub struct Statement
{
    pub name: String,
    pub type_: StatementType,
    pub datatype: Datatype,
    pub statements: Vec<Statement>,
}
#[derive(Clone, Debug)]
pub struct Datatype
{
    pub datatype: StatementDatatype,
    pub array_bounds: Vec<i32>,
    pub is_array: bool,
}
impl Display for Datatype
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
    {
        write!(f, "{}", self.to_string())
    }
}
impl Datatype
{
    pub fn new(datatype: StatementDatatype, array_bounds: Vec<i32>, is_array: bool) -> Datatype
    {
        Datatype
        {
            datatype,
            array_bounds,
            is_array,
        }
    }
    pub fn is_same(&self, other: &Datatype) -> bool
    {
        if self.datatype != other.datatype
        {
            return false;
        }
        if self.is_array != other.is_array
        {
            return false;
        }
        if self.array_bounds.len() != other.array_bounds.len()
        {
            return false;
        }
        for i in 0..self.array_bounds.len()
        {
            if self.array_bounds[i] != other.array_bounds[i]
            {
                return false;
            }
        }
        return true;
    }
    pub fn to_string(&self) -> String
    {
        let mut string = String::new();
        string.push_str(&self.datatype.to_string());
        if self.is_array
        {
            string.push_str("[");
            for bound in &self.array_bounds
            {
                string.push_str(&bound.to_string());
                string.push_str(", ");
            }
            string = string[0..string.len()-2].to_string();
            string.push_str("]");
        }
        return string;
    }
}
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum StatementDatatype
{
    Int,
    Float,
    String,
    Bool,
    Char,
    Void,
}
impl StatementDatatype
{
    pub fn to_string(&self) -> String
    {
        let mut string = String::new();
        match self
        {
            StatementDatatype::Int => string.push_str("int"),
            StatementDatatype::Float => string.push_str("float"),
            StatementDatatype::String => string.push_str("string"),
            StatementDatatype::Bool => string.push_str("bool"),
            StatementDatatype::Char => string.push_str("char"),
            StatementDatatype::Void => string.push_str("void"),
        }
        return string;
    }
}
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum StatementType
{
    Function,
    Variable,
    Class,
    Call,
    Return,
    If,
    Else,
    Literal,
    Assignment,
    Argument,
    Binary,
    Unary,
}
impl StatementType
{
    pub fn to_string(&self) -> String
    {
        let mut string = String::new();
        match self
        {
            StatementType::Function => string.push_str("Function"),
            StatementType::Variable => string.push_str("Variable"),
            StatementType::Class => string.push_str("Class"),
            StatementType::Call => string.push_str("Call"),
            StatementType::Return => string.push_str("Return"),
            StatementType::If => string.push_str("If"),
            StatementType::Else => string.push_str("Else"),
            StatementType::Literal => string.push_str("Literal"),
            StatementType::Assignment => string.push_str("Assignment"),
            StatementType::Argument => string.push_str("Argument"),
            StatementType::Binary => string.push_str("Binary"),
            StatementType::Unary => string.push_str("Unary"),
        }
        return string;
    }
}
impl Statement
{
    pub fn new(name: String, type_: StatementType, datatype: StatementDatatype, array_bounds: Vec<i32>, is_array: bool) -> Statement
    {
        let datatype = Datatype
        {
            datatype,
            array_bounds,
            is_array,
        };
        Statement {
            name,
            type_,
            datatype,
            statements: Vec::<Statement>::new(),
        }
    }
    pub fn new_datatype(name: String, type_: StatementType, datatype: Datatype) -> Statement
    {
        Statement {
            name,
            type_,
            datatype,
            statements: Vec::<Statement>::new(),
        }
    }
    pub fn new_call(name: String, statements: Vec<Statement>, function_datatype: Datatype) -> Statement
    {
        Statement {
            name,
            type_: StatementType::Call,
            datatype: function_datatype,
            statements,
        }
    }
    pub fn to_string(&self) -> String
    {
        format!("{}: {} {}", self.name, self.type_.to_string(), self.datatype.to_string())
    }
    pub fn set_value(&mut self, value: crate::parser::expression::Expression, vars: &HashMap<String, Datatype>, functions: &HashMap<String, (Datatype, Arguments, Vec::<Statement>)>) {
        self.statements.push(util::generate_binary(value, vars, functions));
        // self.statements.push(generate_statement_from_expression(value, &functions));
    }

    pub fn new_return(clone_1: Statement, datatype: Datatype) -> Statement
    {
        let ret = String::from("return");
        Statement {
            name: ret,
            type_: StatementType::Return,
            datatype: datatype,
            statements: vec![clone_1],
        }
    }
    pub fn new_binary(left: Statement, right: Statement, operator: String, datatype: Datatype) -> Statement
    {
        Statement {
            name: operator,
            type_: StatementType::Binary,
            datatype: datatype,
            statements: vec![left, right],
        }
    }

    pub fn new_unary(operand: Statement, operator: String, vec: Datatype) -> Statement {
        Statement
        {
            name: operator,
            type_: StatementType::Unary,
            datatype: vec,
            statements: vec![operand],
        }
    }
}
