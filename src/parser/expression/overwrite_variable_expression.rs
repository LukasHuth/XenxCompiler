use super::Expression;
#[derive(Clone)]
pub struct OverwriteVariableExpression
{
    value: Expression,
    name: String,
}
impl OverwriteVariableExpression
{
    pub fn to_string(&self) -> String
    {
        format!("OverwriteVariableExpression: (name:{} value:{})", self.name, self.value.to_string())
    }
    pub fn new(value: Expression, name: String) -> OverwriteVariableExpression
    {
        OverwriteVariableExpression{value, name}
    }
}