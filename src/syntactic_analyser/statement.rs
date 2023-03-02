pub struct Statement
{
    pub name: String,
    pub type_: StatementType,
    pub datatype: Datatype,
}
pub struct Datatype
{
    pub datatype: StatementDatatype,
    pub array_bounds: Vec<i32>,
    pub is_array: bool,
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
        }
    }
    pub fn to_string(&self) -> String
    {
        todo!();
    }
}