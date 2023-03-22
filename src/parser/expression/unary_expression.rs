use super::Expression;
use super::Token;

#[derive(Clone, PartialEq)]
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

    pub fn get_operator(&self) -> Token {
        return self.operator.clone();
    }

    pub fn get_operand(&self) -> Expression {
        return self.operand.clone();
    }
}