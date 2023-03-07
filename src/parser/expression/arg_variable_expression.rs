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
    pub fn get_type(&self) -> String
    {
        self.type_.clone()
    }
    pub fn get_name(&self) -> String
    {
        self.name.clone()
    }
}