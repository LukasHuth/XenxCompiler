use super::Expression;
#[derive(Clone, PartialEq)]
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

    pub fn get_value(&self) -> Expression {
        return self.value.clone();
    }
}