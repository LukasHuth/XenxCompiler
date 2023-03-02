#[derive(Clone)]
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
}
impl Default for VariableExpression
{
    fn default() -> VariableExpression
    {
        VariableExpression{name: "".to_string()}
    }
}