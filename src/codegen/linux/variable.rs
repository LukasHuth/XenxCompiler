#[derive(Clone, Debug)]
pub struct Variable
{
    pub name: String,
    pub index: usize,
    pub is_argument: bool,
}
impl Variable
{
    pub fn new(name: &str, index: usize, arg: bool) -> Variable
    {
        Variable
        {
            name: name.to_string(),
            index: index,
            is_argument: arg,
        }
    }
    pub fn as_str(&self) -> &str
    {
        self.name.as_str()
    }
}