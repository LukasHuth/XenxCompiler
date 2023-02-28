use std::mem::ManuallyDrop;
use crate::lexer::token::Token;
#[repr(C)]
// #[derive(Clone)]
pub union Syntax
{
    integer_literal: i32,
    boolean_literal: bool,
    string_literal: ManuallyDrop<String>,
    variable_expr: ManuallyDrop<String>,
    binary_expr: ManuallyDrop<BinaryExpression>,
    unary_expr: ManuallyDrop<UnaryExpression>,
    call_expr: ManuallyDrop<CallExpression>,
    assignment_expr: ManuallyDrop<AssignmentExpression>,
    return_expr: ManuallyDrop<ReturnExpression>,
    arg_variable_expr: ManuallyDrop<ArgVariableExpression>,
    function_declaration_expr: ManuallyDrop<FunctionDeclarationExpr>,
    if_expr: ManuallyDrop<IfExpression>,
    overwrite_variable_expr: ManuallyDrop<OverwriteVariableExpression>,
}
#[derive(Clone)]
pub struct BinaryExpression
{
    operator: Token,
    left: Expression,
    right: Expression,
}
#[derive(Clone)]
pub struct IfExpression
{
    condition: Expression,
    then_branch: Vec<Expression>,
    else_branch: Vec<Expression>,
}
#[derive(Clone)]
pub struct ReturnExpression
{
    value: Expression,
}
#[derive(Clone)]
pub struct UnaryExpression
{
    operator: Token,
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
pub struct OverwriteVariableExpression
{
    value: Expression,
    name: String,
}
#[derive(Clone)]
pub struct ArgVariableExpression
{
    type_: String,
    name: String
}
#[derive(Clone)]
pub struct FunctionDeclarationExpr
{
    name: String,
    type_: String,
    args: Vec<Expression>,
    inside: Vec<Expression>,
}
#[derive(Clone)]
pub enum ExpressionTag
{
    IntegerLiteral,
    StringLiteral,
    BooleanLiteral,
    VariableExpr,
    BinaryExpr,
    UnaryExpr,
    CallExpr,
    AssignmentExpr,
    ReturnExpr,
    ArgVariableExpr,
    FunctionDeclarationExpr,
    IfExpr,
    OverwriteVariableExpr,
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
                Syntax { arg_variable_expr } => Syntax {
                    arg_variable_expr: (*arg_variable_expr).clone(),
                },
                Syntax { function_declaration_expr } => Syntax {
                    function_declaration_expr: (*function_declaration_expr).clone(),
                },
                Syntax { if_expr } => Syntax {
                    if_expr: (*if_expr).clone(),
                },
                Syntax { overwrite_variable_expr: override_variable_expr } => Syntax {
                    overwrite_variable_expr: (*override_variable_expr).clone(),
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
    pub fn new_binary_expr(left: Expression, operator: Token, right: Expression) -> Expression
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
    pub fn new_unary_expr(operator: Token, operand: Expression) -> Expression
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
            ExpressionTag::BooleanLiteral => unsafe { syntax.boolean_literal.to_string() },
            ExpressionTag::VariableExpr => unsafe { syntax.variable_expr.to_string() },
            ExpressionTag::BinaryExpr => unsafe { syntax.binary_expr.to_string() },
            ExpressionTag::UnaryExpr => unsafe { syntax.unary_expr.to_string() },
            ExpressionTag::CallExpr => unsafe { syntax.call_expr.to_string() },
            ExpressionTag::AssignmentExpr => unsafe { syntax.assignment_expr.to_string() },
            ExpressionTag::ReturnExpr => unsafe { syntax.return_expr.to_string() },
            ExpressionTag::ArgVariableExpr => unsafe { syntax.arg_variable_expr.to_string() },
            ExpressionTag::FunctionDeclarationExpr => unsafe { syntax.function_declaration_expr.to_string() },
            ExpressionTag::IfExpr => unsafe { syntax.if_expr.to_string() },
            ExpressionTag::OverwriteVariableExpr => unsafe { syntax.overwrite_variable_expr.to_string() },
        }
    }

    pub(crate) fn new_function_expr(_name: String, _type_: String, _args: Vec<Expression>, _inside: Vec<Expression>) -> Expression {
        Expression
        {
            tag: ExpressionTag::FunctionDeclarationExpr,
            syntax: Box::new(Syntax
            {
                function_declaration_expr: ManuallyDrop::new(FunctionDeclarationExpr
                {
                    name: _name,
                    type_: _type_,
                    args: _args,
                    inside: _inside,
                }),
            }),
        }
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
    pub(crate) fn new_arg_variable_expr(name: String, type_: String) -> Expression
    {
        Expression
        {
            tag: ExpressionTag::ArgVariableExpr,
            syntax: Box::new(Syntax
            {
                arg_variable_expr: ManuallyDrop::new(ArgVariableExpression
                {
                    name: name,
                    type_: type_,
                }),
            }),
        }
    }
    pub(crate) fn new_if_expr(condition: Expression, then: Vec<Expression>, else_: Vec<Expression>) -> Expression {
        Expression
        {
            tag: ExpressionTag::IfExpr,
            syntax: Box::new(Syntax
            {
                if_expr: ManuallyDrop::new(IfExpression { condition: condition, then_branch: then, else_branch: else_ }),
            }),
        }
    }
    pub(crate) fn new_overwrite_variable_expression(name: String, value: Expression) -> Expression {
        Expression
        {
            tag: ExpressionTag::OverwriteVariableExpr,
            syntax: Box::new(Syntax
            {
                overwrite_variable_expr: ManuallyDrop::new(OverwriteVariableExpression { name: name, value: value }),
            }),
        }
    }
    pub(crate) fn new_boolean_literal(lit: bool) -> Expression
    {
        Expression
        {
            tag: ExpressionTag::BooleanLiteral,
            syntax: Box::new(Syntax
            {
                boolean_literal: lit,
            })
        }
    }
}
impl BinaryExpression
{
    pub fn to_string(&self) -> String
    {
        format!("BinaryExpression: ({} {} {})", self.left.to_string(), self.operator.text, self.right.to_string())
    }
}
impl UnaryExpression
{
    pub fn to_string(&self) -> String
    {
        format!("UnaryExpression: ({} {})", self.operator.text, self.operand.to_string())
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
impl ArgVariableExpression
{
    pub fn to_string(&self) -> String
    {
        format!("ArgVariableExpression: (name:{} type:{})", self.name, self.type_)
    }
}
impl FunctionDeclarationExpr
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
}
impl OverwriteVariableExpression
{
    pub fn to_string(&self) -> String
    {
        format!("OverwriteVariableExpression: (name:{} value:{})", self.name, self.value.to_string())
    }
}