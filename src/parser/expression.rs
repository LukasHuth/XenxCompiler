use std::mem::ManuallyDrop;
#[repr(C)]
// #[derive(Clone)]
pub union Syntax
{
    integer_literal: i32,
    string_literal: ManuallyDrop<String>,
    variable_expr: ManuallyDrop<String>,
    binary_expr: ManuallyDrop<BinaryExpression>,
    unary_expr: ManuallyDrop<UnaryExpression>,
    call_expr: ManuallyDrop<CallExpression>,
    assignment_expr: ManuallyDrop<AssignmentExpression>,
    return_expr: ManuallyDrop<ReturnExpression>,
}
#[derive(Clone)]
pub struct BinaryExpression
{
    operator: String,
    left: Expression,
    right: Expression,
}
#[derive(Clone)]
pub struct ReturnExpression
{
    value: Expression,
}
#[derive(Clone)]
pub struct UnaryExpression
{
    operator: String,
    operand: Expression,
}
#[derive(Clone)]
pub struct CallExpression
{
    name: String,
    arguments: Vec<Expression>,
}
#[derive(Clone)]
pub struct AssignmentExpression
{
    type_: String,
    value: Expression,
    name: Expression,
}
#[derive(Clone)]
pub enum ExpressionTag
{
    IntegerLiteral,
    StringLiteral,
    VariableExpr,
    BinaryExpr,
    UnaryExpr,
    CallExpr,
    AssignmentExpr,
    ReturnExpr,
}
// #[derive(Clone)]
pub struct Expression
{
    tag: ExpressionTag,
    syntax: Box<Syntax>,
}
impl Clone for Expression
{
    fn clone(&self) -> Self
    {
        Expression
        {
            tag: self.tag.clone(),
            syntax: self.syntax.clone(),
        }
    }
}
#[allow(unreachable_patterns)]
impl Clone for Syntax {
    fn clone(&self) -> Self {
        unsafe {
            match self {
                Syntax { integer_literal } => Syntax {
                    integer_literal: *integer_literal,
                },
                Syntax { string_literal } => Syntax {
                    string_literal: (*string_literal).clone(),
                },
                Syntax { variable_expr } => Syntax {
                    variable_expr: (*variable_expr).clone(),
                },
                Syntax { binary_expr } => Syntax {
                    binary_expr: (*binary_expr).clone(),
                },
                Syntax { unary_expr } => Syntax {
                    unary_expr: (*unary_expr).clone(),
                },
                Syntax { call_expr } => Syntax {
                    call_expr: (*call_expr).clone(),
                },
                Syntax { assignment_expr } => Syntax {
                    assignment_expr: (*assignment_expr).clone(),
                },
                Syntax { return_expr } => Syntax {
                    return_expr: (*return_expr).clone(),
                },
            }
        }
    }
}
#[allow(dead_code)]
impl Expression
{
    pub fn new_integer_literal(value: i32) -> Expression
    {
        Expression
        {
            tag: ExpressionTag::IntegerLiteral,
            syntax: Box::new(Syntax
            {
                integer_literal: value,
            })
        }
    }
    pub fn new_string_literal(value: String) -> Expression
    {
        Expression
        {
            tag: ExpressionTag::StringLiteral,
            syntax: Box::new(Syntax
            {
                string_literal: ManuallyDrop::new(value),
            })
        }
    }
    pub fn new_variable_expr(value:String) -> Expression
    {
        Expression
        {
            tag: ExpressionTag::VariableExpr,
            syntax: Box::new(Syntax
            {
                variable_expr: ManuallyDrop::new(value),
            })
        }
    }
    pub fn new_binary_expr(operator: String, left: Expression, right: Expression) -> Expression
    {
        Expression
        {
            tag: ExpressionTag::BinaryExpr,
            syntax: Box::new(Syntax
            {
                binary_expr: ManuallyDrop::new(BinaryExpression
                {
                    operator: operator,
                    left: left,
                    right: right,
                }),
            })
        }
    }
    pub fn new_unary_expr(operator: String, operand: Expression) -> Expression
    {
        Expression
        {
            tag: ExpressionTag::UnaryExpr,
            syntax: Box::new(Syntax
            {
                unary_expr: ManuallyDrop::new(UnaryExpression
                {
                    operator: operator,
                    operand: operand,
                }),
            })
        }
    }
    pub fn new_call_expr(name: String, arguments: Vec<Expression>) -> Expression
    {
        Expression
        {
            tag: ExpressionTag::CallExpr,
            syntax: Box::new(Syntax
            {
                call_expr: ManuallyDrop::new(CallExpression
                {
                    name: name,
                    arguments: arguments,
                }),
            })
        }
    }
    pub fn new_assignment_expr(name: String, value: Expression, type_: Expression) -> Expression
    {
        Expression
        {
            tag: ExpressionTag::AssignmentExpr,
            syntax: Box::new(Syntax
            {
                assignment_expr: ManuallyDrop::new(AssignmentExpression
                {
                    type_: name,
                    value: value,
                    name: type_,
                }),
            }),
        }
    }
    pub fn to_string(&self) -> String
    {
        let syntax = &self.syntax;
        match self.tag
        {
            ExpressionTag::IntegerLiteral => unsafe { syntax.integer_literal.to_string() },
            ExpressionTag::StringLiteral => unsafe { syntax.string_literal.to_string() },
            ExpressionTag::VariableExpr => unsafe { syntax.variable_expr.to_string() },
            ExpressionTag::BinaryExpr => unsafe { syntax.binary_expr.to_string() },
            ExpressionTag::UnaryExpr => unsafe { syntax.unary_expr.to_string() },
            ExpressionTag::CallExpr => unsafe { syntax.call_expr.to_string() },
            ExpressionTag::AssignmentExpr => unsafe { syntax.assignment_expr.to_string() },
            ExpressionTag::ReturnExpr => unsafe { syntax.return_expr.to_string() },
        }
    }

    pub(crate) fn new_function_expr(_name: String, _type_: String, _args: Vec<Expression>, _inside: Vec<Expression>) -> Expression {
        todo!("Implement function decleration expression");
    }

    pub(crate) fn new_return_expr(number_literal: Expression) -> Expression {
        Expression
        {
            tag: ExpressionTag::ReturnExpr,
            syntax: Box::new(Syntax
            {
                return_expr: ManuallyDrop::new(ReturnExpression
                {
                    value: number_literal,
                }),
            }),
        }
    }
}
impl BinaryExpression
{
    pub fn to_string(&self) -> String
    {
        format!("BinaryExpression: ({} {} {})", self.operator, self.left.to_string(), self.right.to_string())
    }
}
impl UnaryExpression
{
    pub fn to_string(&self) -> String
    {
        format!("UnaryExpression: ({} {})", self.operator, self.operand.to_string())
    }
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
}
impl AssignmentExpression
{
    pub fn to_string(&self) -> String
    {
        format!("AssignmentExpression: (type:{} value:{} name:{})", self.type_, self.value.to_string(), self.name.to_string())
    }
}
impl ReturnExpression
{
    pub fn to_string(&self) -> String
    {
        format!("ReturnExpression: (value:{})", self.value.to_string())
    }
}