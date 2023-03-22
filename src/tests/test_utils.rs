use std::collections::HashMap;

mod lexer_utils;

pub use lexer_utils::*;

use crate::{
    parser::{
        expression::{
            Expression,
            FunctionDeclarationExpression
        },
        Parser
    },
    syntactic_analyser::{
        statement::{
            Statement,
            StatementType,
            StatementDatatype,
            Datatype
        },
        SyntaticAnalyser,
        arguments::Arguments
    },
    lexer::token::{
        Token,
        LexerToken
    }
};


pub fn generate_int_assignment(name: String, value: String) -> Vec<Expression>
{
    let value = value.parse::<i32>().unwrap();
    let literal = Expression::new_integer_literal(value, 0);
    let variable_expr = Expression::new_variable_expr(name, 0);
    let ret = Expression::new_assignment_expr(variable_expr, literal, "int".to_string(), 0);
    return vec![ret];
}
pub fn generate_int_assignment_statement(name: String, value: String) -> Statement
{
    let literal = Statement::new(value, StatementType::Literal, StatementDatatype::Int, vec![], false);
    let datatype = Datatype::new(StatementDatatype::Int, vec![], false);
    // push(Expression::Statement(statement));
    let mut statement = Statement::new(name, StatementType::Variable, datatype.datatype,
        datatype.array_bounds, datatype.is_array);
    statement.statements.push(literal);
    return statement;
}
pub fn syntactic_analyser_get_body(statements: Vec<Expression>) -> Vec<Statement>
{
    let mut syntactic_analyser = SyntaticAnalyser::new(vec![], "".to_string(), false);
    let variables = generate_variables();
    let functiondatatype = generate_function_datatype();
    let args = generate_arguments();
    let ret = syntactic_analyser.get_body(statements, variables, functiondatatype, args, true);
    return ret;
}
pub fn generate_variables() -> HashMap<String, Datatype>
{
    #[allow(unused_mut)]
    let mut variables = HashMap::<String, Datatype>::new();
    // let datatype = StatementDatatype::Int;
    // let array_bounds = Vec::<i32>::new();
    // let datatype = Datatype::new(datatype, array_bounds.clone(), array_bounds.len() > 0);
    // variables.insert("test".to_string(), datatype);
    return variables;
}
pub fn generate_function_datatype() -> Datatype
{
    let datatype = StatementDatatype::Int;
    let array_bounds = Vec::<i32>::new();
    let datatype = Datatype::new(datatype, array_bounds.clone(), array_bounds.len() > 0);
    return datatype;
}
pub fn generate_arguments() -> Arguments
{
    let func_decleration_expression = FunctionDeclarationExpression::new("test".to_string(), "int".to_string(), vec![], vec![]);
    let args: Arguments = crate::syntactic_analyser::util::get_parameters(func_decleration_expression);
    return args;
}
pub fn parse_int_assignment(name: String, value: String) -> Vec<Expression>
{
    let tokens = generate_assignment_tokens(name, value, "int");
    let ret = parse_expressions(tokens);
    return ret;
}
pub fn generate_assignment_tokens(name: String, value: String, datatype: &str) -> Vec<Token>
{
    let mut tokens = Vec::<Token>::new();
    tokens.push(Token::new(LexerToken::Identifier, name.clone(), 0, name.len()));
    tokens.push(Token::new(LexerToken::Colon, ":".to_string(), 0, 1));
    tokens.push(Token::new(LexerToken::Keyword, datatype.to_string(), 0, datatype.len()));
    tokens.push(Token::new(LexerToken::Equals, "=".to_string(), 0, 1));
    tokens.push(Token::new(LexerToken::Literal, value.clone(), 0, value.len()));
    tokens.push(Token::new(LexerToken::Semicolon, ";".to_string(), 0, 1));
    return tokens;
}
pub fn parse_expressions(tokens: Vec<Token>) -> Vec<Expression>
{
    let mut parser = Parser::new(tokens);
    let ret = parser.parse();
    return ret;
}