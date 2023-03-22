use super::Expression;

#[derive(Clone, PartialEq)]
pub struct CallExpression
{
    name: String,
    arguments: Vec<Expression>,
}
impl CallExpression
{
    pub fn to_string(&self) -> String
    {
        let mut args = String::new();
        for arg in &self.arguments
        {
            args.push_str(&arg.to_string());
            args.push_str(", ");
        }
        format!("CallExpression: (name: {} asgs:'{}')", self.name, args)
    }
    pub fn new(name: String, arguments: Vec<Expression>) -> CallExpression
    {
        CallExpression{name, arguments}
    }
    pub fn get_name(&self) -> String
    {
        self.name.clone()
    }
    pub fn get_args(&self) -> Vec<Expression>
    {
        self.arguments.clone()
    }
    pub fn set_name(&mut self, name: String)
    {
        self.name = name;
    }
}