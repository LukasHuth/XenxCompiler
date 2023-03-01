#[derive(Clone)]
pub struct ArgVariableExpression
{
    type_: String,
    name: String
}
impl ArgVariableExpression
{
    pub fn to_string(&self) -> String
    {
        format!("ArgVariableExpression: (name:{} type:{})", self.name, self.type_)
    }
    pub fn new(type_: String, name: String) -> ArgVariableExpression
    {
        ArgVariableExpression{type_, name}
    }
}