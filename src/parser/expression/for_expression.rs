use super::Expression;

#[derive(PartialEq, Clone)]
pub struct ForExpression
{
    initialization_expression: Vec<Expression>,
    test_expression: Expression,
    update_expression:Vec<Expression>,
    body:Vec<Expression>,
}

impl ForExpression
{
    pub fn new(initialization_expression: Vec<Expression>, test_expression: Expression, update_expression: Vec<Expression>, body: Vec<Expression>) -> ForExpression
    {
        ForExpression {
            initialization_expression,
            test_expression,
            update_expression,
            body,
        }
    }
    pub fn get_init_expression(&self) -> Vec<Expression>
    {
        self.initialization_expression.clone()
    }
    pub fn get_test_expression(&self) -> Expression
    {
        self.test_expression.clone()
    }
    pub fn get_update_expression(&self) -> Vec<Expression>
    {
        self.update_expression.clone()
    }
    pub fn get_body(&self) -> Vec<Expression>
    {
        self.body.clone()
    }
    pub fn to_string(&self) -> String
    {
        let mut str = String::new();
        for expr in self.initialization_expression.clone()
        {
            str.push_str(&expr.to_string());
        }
        str.push_str(&self.test_expression.to_string());
        for expr in self.update_expression.clone()
        {
            str.push_str(&expr.to_string());
        }
        for expr in self.body.clone()
        {
            str.push_str(&expr.to_string());
        }
        return str;
    }
}
