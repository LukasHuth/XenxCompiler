use super::Expression;

#[derive(Clone)]
pub struct IfExpression
{
    condition: Expression,
    then_branch: Vec<Expression>,
    else_branch: Vec<Expression>,
}

impl IfExpression
{
    pub fn to_string(&self) -> String
    {
        let mut then = String::new();
        for arg in &self.then_branch
        {
            then.push_str("\t\t");
            then.push_str(&arg.to_string());
            then.push('\n');
        }
        let mut else_ = String::new();
        for arg in &self.else_branch
        {
            else_.push_str("\t\t");
            else_.push_str(&arg.to_string());
            else_.push('\n');
        }
        format!("IfExpression: (\n\tcondition:{} \n\tthen:\n{} \n\telse:\n{})", self.condition.to_string(), then, else_)
    }
    pub fn new(condition: Expression, then_branch: Vec<Expression>, else_branch: Vec<Expression>) -> IfExpression
    {
        IfExpression{condition, then_branch, else_branch}
    }
}