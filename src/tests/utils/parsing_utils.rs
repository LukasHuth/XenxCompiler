use crate::{
    lexer::token::{
        Token,
        LexerToken
    },
    parser::{
        expression::Expression,
        Parser
    },
    syntactic_analyser::statement::{
        StatementType,
        Statement,
        Datatype,
        StatementDatatype
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
fn parse_expressions(tokens: Vec<Token>) -> Vec<Expression>
{
    let mut parser = Parser::new(tokens);
    let ret = parser.parse();
    return ret;
}