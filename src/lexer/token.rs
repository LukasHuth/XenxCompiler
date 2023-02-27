use std::fmt;

use super::Lexer;
pub struct Token
{
    pub token: LexerToken,
    pub text: String
}
#[derive(Debug, PartialEq,Copy,Clone)]
pub enum LexerToken
{
    Openbrace,
    Closebrace,
    Openparenthesis,
    Closeparenthesis,
    Semicolon,
    IntKeyword,
    ReturnKeyword,
    Identifier,
    IntegerLiteral,
    BadToken,
    Less,
    LessEquals,
    Greater,
    GreaterEquals,
    Equals,
    EqualsEquals,
    Bang,
    BangEquals,
    EOF,
}
impl Token
{
    pub fn new(token: LexerToken, text: String) -> Token
    {
        Token
        {
            token: token,
            text: text
        }
    }
    fn token_to_string(&self) -> String
    {
        match self.token
        {
            LexerToken::Openbrace => "Openbrace".to_string(),
            LexerToken::Closebrace => "Closebrace".to_string(),
            LexerToken::Openparenthesis => "Openparenthesis".to_string(),
            LexerToken::Closeparenthesis => "Closeparenthesis".to_string(),
            LexerToken::Semicolon => "Semicolon".to_string(),
            LexerToken::IntKeyword => "IntKeyword".to_string(),
            LexerToken::ReturnKeyword => "ReturnKeyword".to_string(),
            LexerToken::Identifier => "Identifier".to_string(),
            LexerToken::IntegerLiteral => "IntegerLiteral".to_string(),
            LexerToken::EOF => "EOF".to_string(),
            _ => "BadToken".to_string()
        }
    }
    pub fn to_string(&self) -> String
    {
        if self.token == LexerToken::Identifier || self.token == LexerToken::IntegerLiteral
        {
            return self.to_string_with_text();
        }
        return "<Token: ".to_string() + "" + self.token_to_string().as_str() + ">";
    }
    fn to_string_with_text(&self) -> String
    {
        return "<Token: ".to_string() + self.token_to_string().as_str() + " Text: " + &self.text + ">";
    }
    pub fn clone(&self) -> Token
    {
        Token
        {
            token: self.token,
            text: self.text.clone()
        }
    }
}
impl fmt::Display for LexerToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexerToken::Closebrace => write!(f, "Closebrace"),
            LexerToken::Openbrace => write!(f, "Openbrace"),
            LexerToken::Closeparenthesis => write!(f, "Closeparenthesis"),
            LexerToken::Openparenthesis => write!(f, "Openparenthesis"),
            LexerToken::Semicolon => write!(f, "Semicolon"),
            LexerToken::IntKeyword => write!(f, "IntKeyword"),
            LexerToken::ReturnKeyword => write!(f, "ReturnKeyword"),
            LexerToken::Identifier => write!(f, "Identifier"),
            LexerToken::IntegerLiteral => write!(f, "IntegerLiteral"),
            _ => write!(f, "BadToken"),
        }
    }
}