use crate::lexer::token::Token;
use crate::lexer::token::LexerToken;
pub mod expression;
pub use expression::ExpressionSyntax;
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
    pub fn parse(&mut self) -> Vec<ExpressionSyntax>
    {
        let mut statements = Vec::<ExpressionSyntax>::new();
        while !self.is_at_end()
        {
            self.parse_statement(&mut statements);
        }
        return statements;
    }

    fn parse_statement(&mut self, statements: &mut Vec<ExpressionSyntax>) {
        if self.peek().token == LexerToken::Identifier
        {
            let identifier = self.next_token();
            if identifier.text == "if"
            {
                self.match_token(LexerToken::Openparenthesis);
                let expression = self.expression();
                self.match_token(LexerToken::Closeparenthesis);
                let inside = self.parseBracedInstructions();
                let statement = ExpressionSyntax::new_if(identifier, expression, inside);
                statements.push(statement);
            }
        }
        self.next_token();
    }
    fn expression(&mut self) -> ExpressionSyntax
    {
        let left = self.primary();
        let operator = self.next_token();
        println!("Operator: {}", operator.to_string());
        let right = self.primary();
        return ExpressionSyntax::new_binary(operator, left, right);
    }
    fn primary(&mut self) -> ExpressionSyntax
    {
        let token = self.next_token();
        if token.token == LexerToken::Identifier
        {
            return ExpressionSyntax::new_identifier(token);
        }
        else if token.token == LexerToken::IntegerLiteral
        {
            return ExpressionSyntax::new_literal(token);
        }
        else if token.token == LexerToken::Openparenthesis
        {
            let expression = self.expression();
            self.match_token(LexerToken::Closeparenthesis);
            return ExpressionSyntax::new_parenthesized_expression(token, expression);
        }
        panic!("Unexpected token: {}", token.to_string());
    }
    fn match_token(&mut self, token: LexerToken)
    {
        if self.peek().token == token
        {
            self.next_token();
        }
        else
        {
            panic!("Expected token: {}, got: {}", token.to_string(), self.peek().token.to_string());
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

    fn parseBracedInstructions(&mut self) -> ExpressionSyntax {
        let openbrace = self.peek();
        self.match_token(LexerToken::Openbrace);
        let mut statements = Vec::<ExpressionSyntax>::new();
        while !self.is_at_end() && self.peek().token != LexerToken::Closebrace
        {
            self.parse_statement(&mut statements);
        }
        self.match_token(LexerToken::Closebrace);
        return ExpressionSyntax::new_braced_expression(openbrace, statements);
    }
}