#[derive(Clone, Debug)]
pub struct Variable
{
    pub name: String,
    pub index: usize,
}
impl Variable
{
    pub fn new(name: &str, index: usize) -> Variable
    {
        Variable
        {
            name: name.to_string(),
            index: index,
        }
    }
    pub fn as_str(&self) -> &str
    {
        self.name.as_str()
    }
}