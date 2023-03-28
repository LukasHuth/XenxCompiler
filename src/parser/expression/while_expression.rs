use super::Expression;


#[derive(PartialEq, Clone)]
pub struct WhileExpression
{
    pub bool_expr: Expression,
    pub body: Vec<Expression>,
}

impl WhileExpression
{
    pub fn new(bool_expr: Expression, body: Vec<Expression>) -> WhileExpression
    {
        WhileExpression
        {
            bool_expr,
            body,
        }
    }
    pub fn get_bool_expr(&self) -> Expression
    {
        self.bool_expr.clone()
    }
    pub fn body(&self) -> Vec<Expression>
    {
        self.body.clone()
    }
    pub fn to_string(&self) -> String
    {
        let mut str = String::new();
        str.push_str(&self.bool_expr.to_string());
        for expr in self.body.clone()
        {
            str.push_str(&expr.to_string());
        }
        return str;
    }
}
