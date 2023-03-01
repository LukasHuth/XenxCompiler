use super::Expression;
#[derive(Clone)]
pub struct ReturnExpression
{
    value: Expression,
}
impl ReturnExpression
{
    pub fn to_string(&self) -> String
    {
        format!("ReturnExpression: (value:{})", self.value.to_string())
    }
    pub fn new(value: Expression) -> ReturnExpression
    {
        ReturnExpression{value}
    }
}