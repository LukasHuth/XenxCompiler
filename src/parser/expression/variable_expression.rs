use super::Expression;
#[derive(Clone, PartialEq)]
pub struct VariableExpression
{
    name: String,
}

impl VariableExpression
{
    pub fn to_string(&self) -> String
    {
        format!("VariableExpression: ({})", self.name)
    }
    pub fn new(name: String) -> VariableExpression
    {
        VariableExpression{name}
    }
    pub fn get_name(&self) -> String
    {
        self.name.clone()
    }
}
impl Default for VariableExpression
{
    fn default() -> VariableExpression
    {
        VariableExpression{name: "".to_string()}
    }
}
#[derive(Clone, PartialEq)]
pub struct ArrayExpression
{
    name: String,
    index: Vec<Expression>,
}
impl ArrayExpression
{
    pub fn to_string(&self) -> String
    {
        format!("ArrayExpression: ({})", self.name)
    }
    pub fn new(name: String, index: Vec<Expression>) -> ArrayExpression
    {
        ArrayExpression{name, index}
    }
    pub fn get_name(&self) -> String
    {
        self.name.clone()
    }
    pub fn get_index(&self) -> Vec<Expression>
    {
        self.index.clone()
    }
}