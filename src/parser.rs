use crate::lexer::token::Token;
use crate::lexer::token::LexerToken;
pub mod expression;
use expression::Expression;
pub struct Parser
{
    tokens: Vec<Token>,
    current: usize,
}
impl Parser
{
    pub fn new(tokens: Vec<Token>) -> Parser
    {
        Parser
        {
            tokens: tokens,
            current: 0
        }
    }
    // Do this as AST not like this
    pub fn parse(&mut self) -> Vec<Expression>
    {
        let mut statements = Vec::<Expression>::new();
        while !self.is_at_end() && self.peek().token != LexerToken::Closebrace
        {
            if self.peek().token == LexerToken::Identifier
            {
                let identifier = self.next_token();
                // println!("Identifier: {}", self.peek().text);
                if self.peek().token == LexerToken::Colon
                {
                    // println!("Colon: {}", self.peek_off(1).text);
                    self.match_token(LexerToken::Colon);
                    let type_: Token;
                    type_ = self.match_token(LexerToken::Keyword);
                    if !type_.is_data_type()
                    {
                        panic!("Invalid type");
                    }
                    self.match_token(LexerToken::Equals);
                    let value = self.match_token(LexerToken::Literal);
                    println!("Value: {}", value.text);
                    self.match_token(LexerToken::Semicolon);
                    let number_literal = Expression::new_integer_literal(value.text.parse::<i32>().unwrap());
                    let text = String::from(&identifier.text).to_owned();
                    let variable_expr = Expression::new_variable_expr(text);
                    let expression = Expression::new_assignment_expr(type_.text, number_literal, variable_expr);
                    statements.push(expression);
                    // println!("Integer literal: {}", value.text);
                }
                else
                if self.peek().token == LexerToken::Openparenthesis
                {
                    // println!("Function call: {}", identifier.text);
                    self.match_token(LexerToken::Openparenthesis);
                    let mut arguments = Vec::<Expression>::new();
                    while self.peek().token != LexerToken::Closeparenthesis
                    {
                        let argument = self.match_token(LexerToken::Identifier);
                        arguments.push(Expression::new_variable_expr(argument.text));
                        if self.peek().token == LexerToken::Comma
                        {
                            self.match_token(LexerToken::Comma);
                        }
                    }
                    self.match_token(LexerToken::Closeparenthesis);
                    if self.peek().token == LexerToken::Semicolon // Function call
                    {
                        self.match_token(LexerToken::Semicolon);
                        let function_call = Expression::new_call_expr(identifier.text, arguments);
                        statements.push(function_call);
                    }
                }
            }
            else
            if self.peek().token == LexerToken::Keyword
            {
                let key = self.match_token(LexerToken::Keyword);
                if key.text == "return"
                {
                    if self.peek().token == LexerToken::Identifier
                    {
                        let identifier = self.match_token(LexerToken::Identifier);
                        let variable_expr = Expression::new_variable_expr(identifier.text);
                        let expression = Expression::new_return_expr(variable_expr);
                        statements.push(expression);
                        self.match_token(LexerToken::Semicolon);
                        continue;
                    }
                    let value = self.match_token(LexerToken::Literal);
                    self.match_token(LexerToken::Semicolon);
                    let number_literal = Expression::new_integer_literal(value.text.parse::<i32>().unwrap());
                    let expression = Expression::new_return_expr(number_literal);
                    statements.push(expression);
                }
                else
                if key.text == "func"
                {
                    println!("Function definition");
                    let identifier = self.match_token(LexerToken::Identifier);
                    self.match_token(LexerToken::Openparenthesis);
                    let mut arguments = Vec::<Expression>::new();
                    while self.peek().token != LexerToken::Closeparenthesis
                    {
                        let argument = self.match_token(LexerToken::Identifier);
                        self.match_token(LexerToken::Colon);
                        let type_ = self.match_token(LexerToken::Keyword);
                        if !type_.is_data_type()
                        {
                            panic!("Invalid type");
                        }
                        arguments.push(Expression::new_arg_variable_expr(argument.text, type_.text));
                        if self.peek().token != LexerToken::Closeparenthesis
                        {
                            self.match_token(LexerToken::Comma);
                        }
                    }
                    self.match_token(LexerToken::Closeparenthesis);
                    self.match_token(LexerToken::Colon);
                    let type_ = self.match_token(LexerToken::Keyword);
                    if !type_.is_data_type()
                    {
                        panic!("Invalid type");
                    }
                    self.match_token(LexerToken::Arrow);
                    self.match_token(LexerToken::Openbrace);
                    let body = self.parse();
                    self.match_token(LexerToken::Closebrace);
                    let function_definition = Expression::new_function_expr(identifier.text, type_.text, arguments, body);
                    statements.push(function_definition);
                }
            }
            else
            {
                self.next_token();
            }
        }
        return statements;
    }
    fn match_token(&mut self, token: LexerToken) -> Token
    {
        if self.peek().token == token
        {
            return self.next_token();
        }
        else
        {
            panic!("Expected token: {}, got: {}, text: '{}'", token.to_string(), self.peek().token.to_string(), self.peek().text);
        }
    }
    fn next_token(&mut self) -> Token
    {
        if !self.is_at_end()
        {
            self.current += 1;
        }
        self.tokens[self.current-1].clone()
    }
    fn is_at_end(&self) -> bool
    {
        return self.peek().token == crate::lexer::token::LexerToken::EOF;
    }
    fn peek(&self) -> Token
    {
        self.peek_off(0)
    }
    fn peek_off(&self, offset: usize) -> Token
    {
        if self.current + offset >= self.tokens.len()
        {
            return Token::new(LexerToken::EOF, "".to_string());
        }
        return self.tokens[self.current + offset].clone();
    }
}