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

    pub fn get_name(&self) -> String {
        return self.name.clone();
    }
    pub fn get_value(&self) -> Expression {
        return self.value.clone();
    }
}