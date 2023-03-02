use super::Expression;
use super::Token;

#[derive(Clone)]
pub struct BinaryExpression
{
    operator: Token,
    left: Expression,
    right: Expression,
}

impl BinaryExpression
{
    pub fn to_string(&self) -> String
    {
        format!("BinaryExpression: ({} {} {})", self.left.to_string(), self.operator.text, self.right.to_string())
    }
    pub fn new(operator: Token, left: Expression, right: Expression) -> BinaryExpression
    {
        BinaryExpression{operator, left, right}
    }
    pub fn get_operator(&self) -> Token
    {
        self.operator.clone()
    }
    pub fn get_left(&self) -> Expression
    {
        self.left.clone()
    }
    pub fn get_right(&self) -> Expression
    {
        self.right.clone()
    }
}