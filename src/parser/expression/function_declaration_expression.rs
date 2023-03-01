use super::Expression;

#[derive(Clone)]
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
        format!("FunctionDeclarationExpr: (\n\tname:{} \n\ttype:{} \n\targs:\n{} \n\tinside:\n{})", self.name, self.type_, args, inside)
    }
    pub fn new(name: String, type_: String, args: Vec<Expression>, inside: Vec<Expression>) -> FunctionDeclarationExpression
    {
        FunctionDeclarationExpression{name, type_, args, inside}
    }
}