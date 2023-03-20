use super::Token;
mod binary_expression;
mod unary_expression;
mod if_expression;
mod return_expression;
mod call_expression;
mod assignment_expression;
mod overwrite_variable_expression;
mod arg_variable_expression;
mod function_declaration_expression;
mod variable_expression;

pub use binary_expression::BinaryExpression;
pub use unary_expression::UnaryExpression;
pub use if_expression::IfExpression;
pub use return_expression::ReturnExpression;
pub use call_expression::CallExpression;
pub use assignment_expression::AssignmentExpression;
pub use overwrite_variable_expression::OverwriteVariableExpression;
pub use overwrite_variable_expression::OverwriteArrayExpression;
pub use arg_variable_expression::ArgVariableExpression;
pub use function_declaration_expression::FunctionDeclarationExpression;
pub use variable_expression::VariableExpression;

pub mod syntax;
pub use syntax::Syntax;

mod expression_tag;
pub use expression_tag::ExpressionTag;

pub struct Expression
{
    pub tag: ExpressionTag,
    pub syntax: Box<Syntax>,
    pub start: usize,
}
impl Clone for Expression
{
    fn clone(&self) -> Self
    {
        Expression
        {
            tag: self.tag.clone(),
            syntax: self.syntax.clone(),
            start: self.start,
        }
    }
}
#[allow(dead_code)]
impl Expression
{
    #[allow(unreachable_patterns)]
    pub fn to_string(&self) -> String
    {
        let syntax = &self.syntax;
        match self.tag
        {
            ExpressionTag::IntegerLiteral => syntax.integer_literal.unwrap_or(0).to_string(),
            ExpressionTag::StringLiteral => syntax.string_literal.as_ref().unwrap().to_string(),
            ExpressionTag::BooleanLiteral => syntax.boolean_literal.unwrap_or(false).to_string(),
            ExpressionTag::FloatLiteral => syntax.float_literal.unwrap_or(0.0).to_string(),
            ExpressionTag::VariableExpr => syntax.variable_expr.as_ref().unwrap().to_string(),
            ExpressionTag::BinaryExpr => syntax.binary_expr.as_ref().unwrap().to_string(),
            ExpressionTag::UnaryExpr => syntax.unary_expr.as_ref().unwrap().to_string(),
            ExpressionTag::CallExpr => syntax.call_expr.as_ref().unwrap().to_string(),
            ExpressionTag::AssignmentExpr => syntax.assignment_expr.as_ref().unwrap().to_string(),
            ExpressionTag::ReturnExpr => syntax.return_expr.as_ref().unwrap().to_string(),
            ExpressionTag::ArgVariableExpr => syntax.arg_variable_expr.as_ref().unwrap().to_string(),
            ExpressionTag::FunctionDeclarationExpr => syntax.function_declaration_expr.as_ref().unwrap().to_string(),
            ExpressionTag::IfExpr => syntax.if_expr.as_ref().unwrap().to_string(),
            ExpressionTag::OverwriteVariableExpr => syntax.overwrite_variable_expr.as_ref().unwrap().to_string(),
            _ => String::new(),
        }
    }
    pub fn get_position(&self) -> usize
    {
        self.start
    }
    pub fn is_function_declaration(&self) -> bool
    {
        match self.tag
        {
            ExpressionTag::FunctionDeclarationExpr => true,
            _ => false,
        }
    }
    pub fn is_variable_declaration(&self) -> bool
    {
        match self.tag
        {
            ExpressionTag::AssignmentExpr => true,
            _ => false,
        }
    }
    pub fn is_variable_overwrite(&self) -> bool
    {
        match self.tag
        {
            ExpressionTag::OverwriteVariableExpr => true,
            _ => false,
        }
    }
    pub fn is_variable(&self) -> bool
    {
        match self.tag
        {
            ExpressionTag::VariableExpr => true,
            _ => false,
        }
    }
    pub fn is_arg_variable(&self) -> bool
    {
        match self.tag
        {
            ExpressionTag::ArgVariableExpr => true,
            _ => false,
        }
    }
    pub fn is_return(&self) -> bool
    {
        match self.tag
        {
            ExpressionTag::ReturnExpr => true,
            _ => false,
        }
    }
    pub fn is_if(&self) -> bool
    {
        match self.tag
        {
            ExpressionTag::IfExpr => true,
            _ => false,
        }
    }
    pub fn is_call(&self) -> bool
    {
        match self.tag
        {
            ExpressionTag::CallExpr => true,
            _ => false,
        }
    }
    pub fn is_binary(&self) -> bool
    {
        match self.tag
        {
            ExpressionTag::BinaryExpr => true,
            _ => false,
        }
    }   
    pub fn is_unary(&self) -> bool
    {
        match self.tag
        {
            ExpressionTag::UnaryExpr => true,
            _ => false,
        }
    }
    pub fn is_integer_literal(&self) -> bool
    {
        match self.tag
        {
            ExpressionTag::IntegerLiteral => true,
            _ => false,
        }
    }
    pub fn is_string_literal(&self) -> bool
    {
        match self.tag
        {
            ExpressionTag::StringLiteral => true,
            _ => false,
        }
    }
    pub fn is_boolean_literal(&self) -> bool
    {
        match self.tag
        {
            ExpressionTag::BooleanLiteral => true,
            _ => false,
        }
    }
    pub fn is_float_literal(&self) -> bool
    {
        match self.tag
        {
            ExpressionTag::FloatLiteral => true,
            _ => false,
        }
    }
    pub fn is_literal(&self) -> bool
    {
        match self.tag
        {
            ExpressionTag::IntegerLiteral => true,
            ExpressionTag::StringLiteral => true,
            ExpressionTag::BooleanLiteral => true,
            ExpressionTag::FloatLiteral => true,
            _ => false,
        }
    }
    pub fn new_integer_literal(lit: i32, start: usize) -> Expression
    {
        Expression
        {
            tag: ExpressionTag::IntegerLiteral,
            syntax: Box::new(Syntax::new_integer_literal(lit)),
            start,
        }
    }
    pub fn new_string_literal(lit: String, start: usize) -> Expression
    {
        Expression
        {
            tag: ExpressionTag::StringLiteral,
            syntax: Box::new(Syntax::new_string_literal(lit)),
            start,
        }
    }
    pub fn new_boolean_literal(lit: bool, start: usize) -> Expression
    {
        Expression
        {
            tag: ExpressionTag::BooleanLiteral,
            syntax: Box::new(Syntax::new_boolean_literal(lit)),
            start,
        }
    }
    pub fn new_float_literal(lit: f32, start: usize) -> Expression
    {
        Expression
        {
            tag: ExpressionTag::FloatLiteral,
            syntax: Box::new(Syntax::new_float_literal(lit)),
            start,
        }
    }
    pub fn new_variable_expr(value:String, start: usize) -> Expression
    {
        let var_expr = VariableExpression::new(value);
        Expression
        {
            tag: ExpressionTag::VariableExpr,
            syntax: Box::new(Syntax::new_variable_expr(var_expr)),
            start,
        }
    }
    pub fn new_binary_expr(left: Expression, operator: Token, right: Expression, start: usize) -> Expression
    {
        let binary_expr = BinaryExpression::new(operator, left, right);
        Expression
        {
            tag: ExpressionTag::BinaryExpr,
            syntax: Box::new(Syntax::new_binary_expr(binary_expr)),
            start,
        }
    }
    pub fn new_unary_expr(operator: Token, operand: Expression, start: usize) -> Expression
    {
        let unary_expr = UnaryExpression::new(operator, operand);
        Expression
        {
            tag: ExpressionTag::UnaryExpr,
            syntax: Box::new(Syntax::new_unary_expr(unary_expr)),
            start,
        }
    }
    pub fn new_call_expr(name: String, arguments: Vec<Expression>, start: usize) -> Expression
    {
        let call_expr = CallExpression::new(name, arguments);
        Expression
        {
            tag: ExpressionTag::CallExpr,
            syntax: Box::new(Syntax::new_call_expr(call_expr)),
            start,
        }
    }
    pub fn new_assignment_expr(name: String, value: Expression, type_: Expression, start: usize) -> Expression
    {
        let assignment_expr = AssignmentExpression::new(name, value, type_);
        Expression
        {
            tag: ExpressionTag::AssignmentExpr,
            syntax: Box::new(Syntax::new_assignment_expr(assignment_expr)),
            start,
        }
    }
    pub fn new_function_expr(_name: String, _type_: String, _args: Vec<Expression>, _inside: Vec<Expression>, start: usize) -> Expression
    {
        let function_expr = FunctionDeclarationExpression::new(_name, _type_, _args, _inside);
        Expression
        {
            tag: ExpressionTag::FunctionDeclarationExpr,
            syntax: Box::new(Syntax::new_function_declaration_expr(function_expr)),
            start,
        }
    }
    pub fn new_return_expr(value: Expression, start: usize) -> Expression
    {
        let return_expr = ReturnExpression::new(value);
        Expression
        {
            tag: ExpressionTag::ReturnExpr,
            syntax: Box::new(Syntax::new_return_expr(return_expr)),
            start,
        }
    }
    pub fn new_arg_variable_expr(name: String, type_: String, start: usize) -> Expression
    {
        let arg_variable_expr = ArgVariableExpression::new(type_, name);
        Expression
        {
            tag: ExpressionTag::ArgVariableExpr,
            syntax: Box::new(Syntax::new_arg_variable_expr(arg_variable_expr)),
            start,
        }
    }
    pub fn new_if_expr(condition: Expression, then: Vec<Expression>, else_: Vec<Expression>, start: usize) -> Expression
    {
        let if_expr = IfExpression::new(condition, then, else_);
        Expression
        {
            tag: ExpressionTag::IfExpr,
            syntax: Box::new(Syntax::new_if_expr(if_expr)),
            start,
        }
    }
    pub fn new_overwrite_variable_expression(name: String, value: Expression, start: usize) -> Expression
    {
        let overwrite_variable_expr = OverwriteVariableExpression::new(value, name);
        Expression
        {
            tag: ExpressionTag::OverwriteVariableExpr,
            syntax: Box::new(Syntax::new_overwrite_variable_expr(overwrite_variable_expr)),
            start,
        }
    }

    pub fn is_char_literal(&self) -> bool {
        if !self.is_string_literal()
        {
            return false;
        }
        let string_literal = self.syntax.get_string_literal();
        if string_literal.len() != 1
        {
            return false;
        }
        return true;
    }

    pub fn new_array_overwrite_expr(name: String, square_brackets: Vec<Expression>, value: Expression, start: usize) -> Expression
    {
        let overwrite_variable_expr = OverwriteArrayExpression::new(value, name, square_brackets);
        let syntax = Syntax::new_overwrite_array_expr(overwrite_variable_expr);
        let syntax = Box::new(syntax);
        Expression
        {
            tag: ExpressionTag::OverwriteArrayExpr,
            syntax,
            start,
        }
    }

    pub fn is_array_overwrite(&self) -> bool
    {
        match self.tag
        {
            ExpressionTag::OverwriteArrayExpr => true,
            _ => false,
        }
    }
    
}