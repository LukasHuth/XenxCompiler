use super::Expression;
#[derive(Clone)]
pub struct AssignmentExpression
{
    type_: String,
    value: Expression,
    name: Expression,
}
impl AssignmentExpression
{
    pub fn to_string(&self) -> String
    {
        format!("AssignmentExpression: (type:{} value:{} name:{})", self.type_, self.value.to_string(), self.name.to_string())
    }
    pub fn new(type_: String, value: Expression, name: Expression) -> AssignmentExpression
    {
        AssignmentExpression{type_, value, name}
    }
}