use super::Expression;
#[derive(Clone)]
pub struct OverwriteVariableExpression
{
    value: Expression,
    name: String,
}
impl OverwriteVariableExpression
{
    pub fn to_string(&self) -> String
    {
        format!("OverwriteVariableExpression: (name:{} value:{})", self.name, self.value.to_string())
    }
    pub fn new(value: Expression, name: String) -> OverwriteVariableExpression
    {
        OverwriteVariableExpression{value, name}
    }

    pub fn get_name(&self) -> String {
        return self.name.clone();
    }
    pub fn get_value(&self) -> Expression {
        return self.value.clone();
    }
}
#[derive(Clone)]
pub struct OverwriteArrayExpression
{
    value: Expression,
    name: String,
    indices: Vec<Expression>,
}
impl OverwriteArrayExpression
{
    pub fn to_string(&self) -> String
    {
        format!("OverwriteArrayExpression: (name:{} value:{})", self.name, self.value.to_string())
    }
    pub fn new(value: Expression, name: String, indices: Vec<Expression>) -> OverwriteArrayExpression
    {
        OverwriteArrayExpression{value, name, indices}
    }

    pub fn get_name(&self) -> String {
        return self.name.clone();
    }
    pub fn get_value(&self) -> Expression {
        return self.value.clone();
    }
    pub fn get_indices(&self) -> Vec<Expression> {
        return self.indices.clone();
    }
}