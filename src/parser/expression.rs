use std::mem::ManuallyDrop;
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

use binary_expression::BinaryExpression;
use unary_expression::UnaryExpression;
use if_expression::IfExpression;
use return_expression::ReturnExpression;
use call_expression::CallExpression;
use assignment_expression::AssignmentExpression;
use overwrite_variable_expression::OverwriteVariableExpression;
use arg_variable_expression::ArgVariableExpression;
use function_declaration_expression::FunctionDeclarationExpression;
use variable_expression::VariableExpression;

mod syntax;
use syntax::Syntax;

mod expression_tag;
use expression_tag::ExpressionTag;

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
#[allow(dead_code)]
impl Expression
{
    pub fn to_string(&self) -> String
    {
        let syntax = &self.syntax;
        match self.tag
        {
            ExpressionTag::IntegerLiteral => unsafe { syntax.integer_literal.to_string() },
            ExpressionTag::StringLiteral => unsafe { syntax.string_literal.to_string() },
            ExpressionTag::BooleanLiteral => unsafe { syntax.boolean_literal.to_string() },
            ExpressionTag::FloatLiteral => unsafe { syntax.float_literal.to_string() },
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
    pub(crate) fn new_float_literal(lit: f32) -> Expression
    {
        Expression
        {
            tag: ExpressionTag::FloatLiteral,
            syntax: Box::new(Syntax
            {
                float_literal: lit,
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
                variable_expr: ManuallyDrop::new(VariableExpression::new(value)),
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
                binary_expr: ManuallyDrop::new(BinaryExpression::new
                (
                    operator,
                    left,
                    right,
                )),
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
                unary_expr: ManuallyDrop::new(UnaryExpression::new
                (
                    operator,
                    operand,
                )),
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
                call_expr: ManuallyDrop::new(CallExpression::new
                (
                    name,
                    arguments,
                )),
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
                assignment_expr: ManuallyDrop::new(AssignmentExpression::new
                (
                    name,
                    value,
                    type_,
                )),
            }),
        }
    }
    pub(crate) fn new_function_expr(_name: String, _type_: String, _args: Vec<Expression>, _inside: Vec<Expression>) -> Expression
    {
        Expression
        {
            tag: ExpressionTag::FunctionDeclarationExpr,
            syntax: Box::new(Syntax
            {
                function_declaration_expr: ManuallyDrop::new(FunctionDeclarationExpression::new
                (
                    _name,
                    _type_,
                    _args,
                    _inside,
                )),
            }),
        }
    }
    pub(crate) fn new_return_expr(number_literal: Expression) -> Expression
    {
        Expression
        {
            tag: ExpressionTag::ReturnExpr,
            syntax: Box::new(Syntax
            {
                return_expr: ManuallyDrop::new(ReturnExpression::new
                (
                    number_literal,
                )),
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
                arg_variable_expr: ManuallyDrop::new(ArgVariableExpression::new
                (
                    name,
                    type_,
                )),
            }),
        }
    }
    pub(crate) fn new_if_expr(condition: Expression, then: Vec<Expression>, else_: Vec<Expression>) -> Expression
    {
        Expression
        {
            tag: ExpressionTag::IfExpr,
            syntax: Box::new(Syntax
            {
                if_expr: ManuallyDrop::new(IfExpression::new( condition, then, else_ )),
            }),
        }
    }
    pub(crate) fn new_overwrite_variable_expression(name: String, value: Expression) -> Expression
    {
        Expression
        {
            tag: ExpressionTag::OverwriteVariableExpr,
            syntax: Box::new(Syntax
            {
                overwrite_variable_expr: ManuallyDrop::new(OverwriteVariableExpression::new(value, name)),
            }),
        }
    }
    
}