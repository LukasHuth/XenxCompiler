
use super::lexer::token::{
    Token,
    LexerToken
};
pub mod expression;
#[path ="tests/parser_tests.rs"]
mod unit_tests;
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
    pub fn parse_square_bracket(&mut self, datatype: &mut String)
    {
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
            *datatype = format!("{}[{}]", datatype, numer);
        }
    }
    pub fn parse_identifier(&mut self, identifier: Token, statements: &mut Vec<Expression>, start: usize)
    {
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
            self.parse_square_bracket(&mut datatype);
            if self.peek().token != LexerToken::Equals
            {
                let new = Expression::new_integer_literal(0, start);
                let text = String::from(&identifier.text).to_owned();
                let variable_expr = Expression::new_variable_expr(text, start);
                // let expression = Expression::new_assignment_expr(datatype.to_owned(), new, variable_expr, start);
                let expression = Expression::new_assignment_expr(variable_expr, new, datatype.to_owned(), start);
                statements.push(expression);
            }
            else
            {
                self.match_token(LexerToken::Equals);
                let expr = self.parse_expression();
                self.match_token(LexerToken::Semicolon);
                let text = String::from(&identifier.text).to_owned();
                let variable_expr = Expression::new_variable_expr(text, start);
                // let expression = Expression::new_assignment_expr(datatype, expr, variable_expr, start);
                let expression = Expression::new_assignment_expr(variable_expr, expr, datatype, start);
                statements.push(expression);
            }
            return;
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
            return;
        }
        let mut square_brackets = Vec::<Expression>::new();
        while self.peek().token == LexerToken::OpenSquareBracket
        {
            self.match_token(LexerToken::OpenSquareBracket);
            let expr = self.parse_expression();
            self.match_token(LexerToken::CloseSquareBracket);
            square_brackets.push(expr);
        }
        if self.peek().token == LexerToken::Equals
        {
            self.match_token(LexerToken::Equals);
            let value = self.parse_expression();
            // println!("test 1");
            self.match_token(LexerToken::Semicolon);
            let text = String::from(&identifier.text).to_owned();
            let expression: Expression;
            if square_brackets.len() > 0
            {
                expression = Expression::new_array_overwrite_expr(text.clone(), square_brackets, value, start);
            }
            else
            {
                expression = Expression::new_overwrite_variable_expression(text.clone(), value, start);
            }
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
            let value = Expression::new_binary_expr(left, operator, expr, start);
            let expression: Expression;
            if square_brackets.len() > 0
            {
                expression = Expression::new_array_overwrite_expr(text.clone(), square_brackets, value, start);
            }
            else
            {
                expression = Expression::new_overwrite_variable_expression(text.clone(), value, start);
            }
            statements.push(expression);
        }
    }
    pub fn parse(&mut self, namespace_name: &str) -> Vec<Expression>
    {
        let mut statements = Vec::<Expression>::new();
        while !self.is_at_end() && self.peek().token != LexerToken::Closebrace
        {
            let start = self.peek().pos;
            if self.peek().token == LexerToken::Identifier
            {
                let identifier = self.match_token(LexerToken::Identifier);
                let identifier = self.get_identifier(identifier);
                self.parse_identifier(identifier, &mut statements, start);
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
                    // println!("Expected ':'");
                    self.match_token(LexerToken::Colon);
                    // println!("Found!");
                    let type_ = self.match_token(LexerToken::Keyword);
                    if !type_.is_data_type()
                    {
                        panic!("Invalid type");
                    }
                    self.match_token(LexerToken::Arrow);
                    self.match_token(LexerToken::Openbrace);
                    let body = self.parse(namespace_name.clone());
                    self.match_token(LexerToken::Closebrace);
                    let name_str = identifier.text;
                    let name: String;
                    if namespace_name == ""
                    {
                        name = name_str;
                    }
                    else
                    {
                        name = namespace_name.to_owned()+"::"+&name_str;
                    }
                    let function_definition = Expression::new_function_expr(name, type_.text, arguments, body, start);
                    statements.push(function_definition);
                }
                else if key.text == "if"
                {
                    let if_statement = self.parse_if_statement(false, namespace_name);
                    statements.push(if_statement);
                }
                else if key.text == "namespace"
                {
                    let namespace = self.match_token(LexerToken::Identifier);
                    let name = namespace.text;
                    let mut name = namespace_name.to_owned()+"::"+&name;
                    if namespace_name == ""
                    {
                        name = name[2..].to_owned();
                    }
                    self.match_token(LexerToken::Openbrace);
                    let mut body = self.parse(&name);
                    self.match_token(LexerToken::Closebrace);
                    statements.append(&mut body);
                }
                else if key.text == "for"
                {
                    self.parse_for_definition(&mut statements, namespace_name);
                }
            }
            else
            {
                self.next_token();
            }
        }
        return statements;
    }
    fn parse_for_definition(&mut self,statements: &mut Vec<Expression>, namespace_name: &str)
    {
        self.match_token(LexerToken::Openparenthesis);
        let identifier = self.match_token(LexerToken::Identifier);
        let start = identifier.pos;
        self.parse_identifier(identifier, statements, start);
        let start_expression = start;
        let bool_expression = self.parse_expression();
        let op_expression = self.parse_expression();
        self.match_token(LexerToken::Closeparenthesis);
        self.match_token(LexerToken::Arrow);
        self.match_token(LexerToken::Openbrace);
        let body = self.parse(namespace_name);
        self.match_token(LexerToken::Closebrace);
        // let for_expression = Expression::new_for_epxression(start_expression,bool_expression,op_expression,body);
        // statements.push(for_expression);
    }
    fn get_identifier(&mut self, identifier: Token) -> Token {
        if self.peek_off(0).token == LexerToken::Colon && self.peek_off(1).token == LexerToken::Colon && self.peek_off(2).token == LexerToken::Identifier
        {
            let identifier1 = identifier;
            self.match_token(LexerToken::Colon);
            self.match_token(LexerToken::Colon);
            let identifier2 = self.match_token(LexerToken::Identifier);
            let result = Token::new(LexerToken::Identifier, format!("{}::{}", identifier1.text, identifier2.text), identifier1.pos, identifier1.length + identifier2.length + 2);
            return self.get_identifier(result);
        }
        else{
            return identifier;
        }
    }
    fn parse_if_statement(&mut self, inside: bool, namespace_name: &str) -> Expression
    {
        let start = self.peek().pos - 2;
        self.match_token(LexerToken::Openparenthesis);
        let condition = self.parse_expression();
        self.match_token(LexerToken::Closeparenthesis);
        self.match_token(LexerToken::Openbrace);
        let body = self.parse(namespace_name.clone());
        self.match_token(LexerToken::Closebrace);
        if self.peek().token == LexerToken::Keyword && self.peek().text == "else"
        {
            self.match_token(LexerToken::Keyword);
            if self.peek().token == LexerToken::Openbrace
            {
                self.match_token(LexerToken::Openbrace);
                let else_body = self.parse(namespace_name.clone());
                if !inside
                {
                    self.match_token(LexerToken::Closebrace);
                }
                return Expression::new_if_expr(condition, body, else_body, start);
            }
            else
            {
                self.match_token(LexerToken::Keyword);
                let else_if_statement = self.parse_if_statement(true, namespace_name.clone());
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
                let float_lit = literal.text.parse::<f32>().unwrap();
                // println!("Float literal: {}", float_lit);
                return Expression::new_float_literal(float_lit, start);
            }
            panic!("Invalid type of literal: {}", literal.text)
        }
        if self.peek().token == LexerToken::Identifier
        {
            let identifier = self.match_token(LexerToken::Identifier);
            let identifier = self.get_identifier(identifier);
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
            let mut square_brackets = Vec::<Expression>::new();
            while self.peek().token == LexerToken::OpenSquareBracket
            {
                self.match_token(LexerToken::OpenSquareBracket);
                let expr = self.parse_expression();
                self.match_token(LexerToken::CloseSquareBracket);
                square_brackets.push(expr);
            }
            if square_brackets.len() > 0
            {
                return Expression::new_array_access_expr(identifier.text, square_brackets, start);
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
            let token = self.next_token();
            print!("{} ", token.text);
            return token;
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
