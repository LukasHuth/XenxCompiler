
use super::lexer::token::{
    Token,
    LexerToken
};
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
            current: 0,
        }
    }
    // Do this as AST not like this
    pub fn parse(&mut self) -> Vec<Expression>
    {
        let mut statements = Vec::<Expression>::new();
        while !self.is_at_end() && self.peek().token != LexerToken::Closebrace
        {
            let start = self.peek().pos;
            if self.peek().token == LexerToken::Identifier
            {
                let identifier = self.next_token();
                if self.peek().token == LexerToken::Colon
                {
                    self.match_token(LexerToken::Colon);
                    let type_: Token;
                    type_ = self.match_token(LexerToken::Keyword);
                    if !type_.is_data_type()
                    {
                        panic!("Invalid type");
                    }
                    let mut datatype = type_.text;
                    while self.peek().token == LexerToken::OpenSquareBracket
                    {
                        self.match_token(LexerToken::OpenSquareBracket);
                        // println!("{}", self.peek().text);
                        let numer = self.match_token(LexerToken::Literal);
                        if !numer.text.parse::<i32>().is_ok()
                        {
                            panic!("Invalid array size (Array size has to be i32 not '{}')", numer.text);
                        }
                        let numer = numer.text.parse::<i32>().unwrap();
                        self.match_token(LexerToken::CloseSquareBracket);
                        datatype = format!("{}[{}]", datatype, numer);
                    }
                    if self.peek().token != LexerToken::Equals
                    {
                        let new = Expression::new_integer_literal(0, start);
                        let text = String::from(&identifier.text).to_owned();
                        let variable_expr = Expression::new_variable_expr(text, start);
                        let expression = Expression::new_assignment_expr(datatype.to_owned(), new, variable_expr, start);
                        statements.push(expression);
                    }
                    else
                    {
                        self.match_token(LexerToken::Equals);
                        let expr = self.parse_expression();
                        self.match_token(LexerToken::Semicolon);
                        let text = String::from(&identifier.text).to_owned();
                        let variable_expr = Expression::new_variable_expr(text, start);
                        let expression = Expression::new_assignment_expr(datatype, expr, variable_expr, start);
                        statements.push(expression);
                    }
                    continue;
                }
                else
                if self.peek().token == LexerToken::Openparenthesis
                {
                    self.match_token(LexerToken::Openparenthesis);
                    let mut arguments = Vec::<Expression>::new();
                    while self.peek().token != LexerToken::Closeparenthesis
                    {
                        let argument = self.parse_expression();
                        arguments.push(argument);
                        if self.peek().token == LexerToken::Comma
                        {
                            self.match_token(LexerToken::Comma);
                        }
                    }
                    self.match_token(LexerToken::Closeparenthesis);
                    if self.peek().token == LexerToken::Semicolon // Function call
                    {
                        self.match_token(LexerToken::Semicolon);
                        let function_call = Expression::new_call_expr(identifier.text, arguments, start);
                        statements.push(function_call);
                    }
                    continue;
                }
                let mut square_brackets = Vec::<Expression>::new();
                while self.peek().token == LexerToken::OpenSquareBracket
                {
                    self.match_token(LexerToken::OpenSquareBracket);
                    let expr = self.parse_expression();
                    self.match_token(LexerToken::CloseSquareBracket);
                    square_brackets.push(expr);
                }
                // TODO: Add support for arrays
                if self.peek().token == LexerToken::Equals
                {
                    self.match_token(LexerToken::Equals);
                    let expr = self.parse_expression();
                    self.match_token(LexerToken::Semicolon);
                    let text = String::from(&identifier.text).to_owned();
                    let expression = Expression::new_overwrite_variable_expression(text, expr, start);
                    statements.push(expression);
                }
                else
                if (self.peek().token == LexerToken::Plus || self.peek().token == LexerToken::Minus || self.peek().token == LexerToken::Star ||
                    self.peek().token == LexerToken::Slash || self.peek().token == LexerToken::Percent || self.peek().token == LexerToken::Caret ||
                    self.peek().token == LexerToken::Ampersand || self.peek().token == LexerToken::Pipe)
                    && self.peek_off(1).token == LexerToken::Equals
                {
                    let operator = self.next_token();
                    self.match_token(LexerToken::Equals);
                    let expr = self.parse_expression();
                    self.match_token(LexerToken::Semicolon);
                    let text = String::from(&identifier.text).to_owned();
                    let left = Expression::new_variable_expr(text.clone(), start);
                    let expression = Expression::new_binary_expr(left, operator, expr, start);
                    statements.push(Expression::new_overwrite_variable_expression(text.clone(), expression, start));
                }
            }
            else
            if self.peek().token == LexerToken::Keyword
            {
                let key = self.match_token(LexerToken::Keyword);
                if key.text == "return"
                {
                    let expr = self.parse_expression();
                    self.match_token(LexerToken::Semicolon);
                    let expression = Expression::new_return_expr(expr, start);
                    statements.push(expression);
                }
                else
                if key.text == "func"
                {
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
                        let mut inside = String::new();
                        if self.peek().token == LexerToken::OpenSquareBracket
                        {
                            while self.peek().token == LexerToken::OpenSquareBracket
                            {
                                self.match_token(LexerToken::OpenSquareBracket);
                                self.match_token(LexerToken::CloseSquareBracket);
                                inside.push_str("[]");
                            }
                        }
                        arguments.push(Expression::new_arg_variable_expr(argument.text, type_.text + &inside, start));
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
                    let function_definition = Expression::new_function_expr(identifier.text, type_.text, arguments, body, start);
                    statements.push(function_definition);
                }
                else if key.text == "if"
                {
                    let if_statement = self.parse_if_statement(false);
                    statements.push(if_statement);
                }
            }
            else
            {
                self.next_token();
            }
        }
        return statements;
    }
    fn parse_if_statement(&mut self, inside: bool) -> Expression
    {
        let start = self.peek().pos - 2;
        self.match_token(LexerToken::Openparenthesis);
        let condition = self.parse_expression();
        self.match_token(LexerToken::Closeparenthesis);
        self.match_token(LexerToken::Openbrace);
        let body = self.parse();
        self.match_token(LexerToken::Closebrace);
        if self.peek().token == LexerToken::Keyword && self.peek().text == "else"
        {
            self.match_token(LexerToken::Keyword);
            if self.peek().token == LexerToken::Openbrace
            {
                self.match_token(LexerToken::Openbrace);
                let else_body = self.parse();
                if !inside
                {
                    self.match_token(LexerToken::Closebrace);
                }
                return Expression::new_if_expr(condition, body, else_body, start);
            }
            else
            {
                self.match_token(LexerToken::Keyword);
                let else_if_statement = self.parse_if_statement(true);
                self.match_token(LexerToken::Closebrace);
                return Expression::new_if_expr(condition, body, vec![else_if_statement], start);
            }
        }
        Expression::new_if_expr(condition, body, Vec::new(), start)
    }
    fn parse_binary_expression(&mut self, precedence: i32) -> Expression
    {
        let start = self.peek().pos;
        let mut left = self.parse_unary_expression();
        while precedence < self.peek().precedence()
        {
            let operator = self.next_token();
            let right = self.parse_binary_expression(operator.precedence());
            left = Expression::new_binary_expr(left, operator, right, start);
        }
        // println!("Returning left: {}", left.to_string());
        return left;
    }
    fn parse_unary_expression(&mut self) -> Expression
    {
        let start = self.peek().pos;
        if self.peek().token == LexerToken::Minus
        {
            let operator = self.match_token(LexerToken::Minus);
            let right = self.parse_unary_expression();
            return Expression::new_unary_expr(operator, right, start);
        }
        if self.peek().token == LexerToken::Bang
        {
            let operator = self.match_token(LexerToken::Bang);
            let right = self.parse_unary_expression();
            return Expression::new_unary_expr(operator, right, start);
        }
        return self.parse_primary_expression();
    }
    fn parse_primary_expression(&mut self) -> Expression
    {
        let start = self.peek().pos;
        if self.peek().token == LexerToken::Literal
        {
            let literal = self.match_token(LexerToken::Literal);
            if literal.is_boolean()
            {
                return Expression::new_boolean_literal(literal.text.parse::<bool>().unwrap(), start);
            }
            if literal.is_string()
            {
                return Expression::new_string_literal(literal.text, start);
            }
            if literal.is_integer()
            {
                return Expression::new_integer_literal(literal.text.parse::<i32>().unwrap(), start);
            }
            if literal.is_float()
            {
                return Expression::new_float_literal(literal.text.parse::<f32>().unwrap(), start);
            }
            panic!("Invalid type of literal: {}", literal.text)
        }
        if self.peek().token == LexerToken::Identifier
        {
            let identifier = self.match_token(LexerToken::Identifier);
            if self.peek().token == LexerToken::Openparenthesis
            {
                self.match_token(LexerToken::Openparenthesis);
                let mut arguments = Vec::new();
                while self.peek().token != LexerToken::Closeparenthesis
                {
                    let argument = self.parse_expression();
                    arguments.push(argument);
                    if self.peek().token != LexerToken::Closeparenthesis
                    {
                        self.match_token(LexerToken::Comma);
                    }
                }
                self.match_token(LexerToken::Closeparenthesis);
                return Expression::new_call_expr(identifier.text, arguments, start);
            }
            return Expression::new_variable_expr(identifier.text, start);
        }
        if self.peek().token == LexerToken::Openparenthesis
        {
            self.match_token(LexerToken::Openparenthesis);
            let expression = self.parse_expression();
            self.match_token(LexerToken::Closeparenthesis);
            return expression;
        }
        panic!("Invalid expression got: {}, text: '{}'", self.peek().token, self.peek().text);
    }
    fn parse_expression(&mut self) -> Expression
    {
        return self.parse_binary_expression(0);
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
        return self.peek().token == LexerToken::EOF;
    }
    fn peek(&self) -> Token
    {
        self.peek_off(0)
    }
    fn peek_off(&self, offset: usize) -> Token
    {
        if self.current + offset >= self.tokens.len()
        {
            return Token::new(LexerToken::EOF, "".to_string(), 0, 0);
        }
        return self.tokens[self.current + offset].clone();
    }
}
