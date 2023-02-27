
use crate::lexer::token::Token;
use std::fmt;
pub struct ExpressionSyntax
{
    pub expression: ExpressionType,
    pub text: String,
    pub children: Vec<ExpressionSyntax>,
}
pub enum ExpressionType
{
    Assignment,
    IfStatement,
    BinaryOperation,
    ParenthesizedExpression,
    BracedExpression,
    Literal,
    Identifier,
}
#[allow(dead_code)]
#[allow(unused_variables)]
impl ExpressionSyntax
{
    pub fn new(expression: ExpressionType, text: String, children: Vec<ExpressionSyntax>) -> ExpressionSyntax
    {
        ExpressionSyntax
        {
            expression: expression,
            text: text,
            children: children
        }
    }
    pub fn to_string(&self) -> String
    {
        let mut chidlstring = String::new();
        for i in 0..self.children.len()
        {
            chidlstring = chidlstring + self.children[i].to_string_child(1).as_str();
        }
        return "<Expression: ".to_string() + self.expression.to_string().as_str() + ">\n"+chidlstring.as_str();
    }
    pub fn to_string_child(&self, index: usize) -> String
    {
        let mut chidlstring = String::new();
        for i in 0..self.children.len()
        {
            chidlstring = chidlstring + self.children[i].to_string_child(index+1).as_str() + "\n";
        }
        return " ".repeat(index*3) + "<Expression: " + self.expression.to_string().as_str() + ">\n"+chidlstring.as_str();
    }
    pub fn new_assignment(token: Token, left: ExpressionSyntax, right: ExpressionSyntax) -> ExpressionSyntax
    {
        let children = vec![left, right];
        Self::new(ExpressionType::Assignment, token.text, children)
    }
    pub fn new_if(token: Token, inside: ExpressionSyntax, baced_instructions: ExpressionSyntax) -> ExpressionSyntax
    {
        let children = vec![inside, baced_instructions];
        Self::new(ExpressionType::IfStatement, token.text, children)
    }
    pub fn new_binary(token: Token, left: ExpressionSyntax, right: ExpressionSyntax) -> ExpressionSyntax
    {
        let children = vec![left, right];
        Self::new(ExpressionType::BinaryOperation, token.text, children)
    }
    pub fn new_literal(token: Token) -> ExpressionSyntax
    {
        Self::new(ExpressionType::Literal, token.text, Vec::new())
    }
    pub fn new_identifier(token: Token) -> ExpressionSyntax
    {
        Self::new(ExpressionType::Identifier, token.text, Vec::new())
    }
    pub fn new_parenthesized_expression(token: Token, inside: ExpressionSyntax) -> ExpressionSyntax
    {
        let children = vec![inside];
        Self::new(ExpressionType::ParenthesizedExpression, token.text, children)
    }
    pub fn new_braced_expression(token: Token, inside: Vec::<ExpressionSyntax>) -> ExpressionSyntax
    {
        let children = inside;
        Self::new(ExpressionType::BracedExpression, token.text, children)
    }
}
impl fmt::Display for ExpressionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExpressionType::Assignment => write!(f, "Assignment"),
            ExpressionType::IfStatement => write!(f, "IfStatement"),
            ExpressionType::BinaryOperation => write!(f, "BinaryOperation"),
            ExpressionType::ParenthesizedExpression => write!(f, "ParenthesizedExpression"),
            ExpressionType::BracedExpression => write!(f, "BracedExpression"),
            ExpressionType::Literal => write!(f, "Literal"),
            ExpressionType::Identifier => write!(f, "Identifier"),
            _ => write!(f, "BadToken"),
        }
    }
}