use super::Expression;

#[derive(Clone, PartialEq)]
pub struct FunctionDeclarationExpression
{
    name: String,
    type_: String,
    args: Vec<Expression>,
    inside: Vec<Expression>,
}
impl FunctionDeclarationExpression
{
    pub fn to_string(&self) -> String
    {
        let mut args = String::new();
        for arg in &self.args
        {
            args.push_str("\t\t");
            args.push_str(&arg.to_string());
            args.push('\n');
        }
        let mut inside = String::new();
        for arg in &self.inside
        {
            inside.push_str("\t\t");
            inside.push_str(&arg.to_string());
            inside.push('\n');
        }
        format!("FunctionDeclarationExpr: (\n\tname:{} \n\ttype:{} \n\targs:\n[{}] \n\tinside:\n[{}])", self.name, self.type_, args, inside)
    }
    pub fn new(name: String, type_: String, args: Vec<Expression>, inside: Vec<Expression>) -> FunctionDeclarationExpression
    {
        FunctionDeclarationExpression{name, type_, args, inside}
    }
    pub fn get_name(&self) -> String
    {
        self.name.clone()
    }
    pub fn get_type(&self) -> String
    {
        self.type_.clone()
    }
    pub fn get_args(&self) -> Vec<Expression>
    {
        self.args.clone()
    }
    pub fn get_inside(&self) -> Vec<Expression>
    {
        self.inside.clone()
    }
}