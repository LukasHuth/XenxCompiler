use super::super::Datatype;
#[derive(Clone, Debug)]
pub struct Variable
{
    pub name: String,
    pub index: usize,
    pub is_argument: bool,
    pub datatype: Datatype,
    pub is_string: bool,
}
impl Variable
{
    pub fn new(name: &str, index: usize, arg: bool, datatype: Datatype) -> Variable
    {
        Variable
        {
            name: name.to_string(),
            index: index,
            is_argument: arg,
            datatype: datatype,
            is_string: false,
        }
    }
    pub fn new_string(name: &str, index: usize, arg: bool, datatype: Datatype) -> Variable
    {
        Variable
        {
            name: name.to_string(),
            index: index,
            is_argument: arg,
            datatype: datatype,
            is_string: true,
        }
    }
    pub fn as_str(&self) -> &str
    {
        self.name.as_str()
    }
}