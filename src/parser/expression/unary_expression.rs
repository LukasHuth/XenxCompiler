use super::Expression;
use super::Token;

#[derive(Clone)]
pub struct UnaryExpression
{
    operator: Token,
    operand: Expression,
}
impl UnaryExpression
{
    pub fn to_string(&self) -> String
    {
        format!("UnaryExpression: ({} {})", self.operator.text, self.operand.to_string())
    }
    pub fn new(operator: Token, operand: Expression) -> UnaryExpression
    {
        UnaryExpression{operator, operand}
    }
}