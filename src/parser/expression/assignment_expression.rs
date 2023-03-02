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
    pub fn get_type(&self) -> String
    {
        self.type_.clone()
    }
    pub fn get_value(&self) -> Expression
    {
        self.value.clone()
    }
    pub fn get_name(&self) -> Expression
    {
        self.name.clone()
    }
}