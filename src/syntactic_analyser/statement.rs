use std::fmt::Formatter;
use std::fmt::Error;
use std::fmt::Display;
pub struct Statement
{
    pub name: String,
    pub type_: StatementType,
    pub datatype: Datatype,
    pub statements: Vec<Statement>,
}
#[derive(Clone)]
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
#[derive(PartialEq, Copy, Clone)]
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
        todo!();
    }
}