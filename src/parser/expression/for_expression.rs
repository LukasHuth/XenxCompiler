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
}
