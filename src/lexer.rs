pub mod token;
use token::{
    LexerToken,
    Token,
};

use crate::naming_util;
pub struct Lexer {
    input: String,
    position: usize,
}
#[path ="tests/lexer_tests.rs"]
pub mod lexer_tests;
impl Lexer
{
    pub fn new(input: String) -> Lexer {
        Lexer { input: input, position: 0 }
    }
    pub fn lex(&mut self) -> Vec<Token>
    {
        let mut tokens = Vec::<Token>::new();
        while self.peek(0) != '\0'
        {
            let token = self.lex_token();
            tokens.push(token);
        }
        return tokens;
    }
    pub fn skip_whitespace(&mut self)
    {
        while self.peek(0) == ' ' || self.peek(0) == '\t' || self.peek(0) == '\r' || self.peek(0) == '\n'
        {
            self.next();
        }
    }
    pub fn lex_token(&mut self) -> Token
    {
        let start = self.position;
        let current = self.next();
        let token: LexerToken;
        let mut lexnext = false;
        match current
        {
            ' ' | '\t' | '\r' | '\n' => {
                self.skip_whitespace();
                token = LexerToken::BadToken;
                lexnext = true;
            },
            '{' => {
                token = LexerToken::Openbrace;
            },
            '}' => {
                token = LexerToken::Closebrace;
            },
            '[' => {
                token = LexerToken::OpenSquareBracket;
            },
            ']' => {
                token = LexerToken::CloseSquareBracket;
            },
            '+' => {
                token = LexerToken::Plus;
            },
            '-' => {
                token = LexerToken::Minus;
            },
            '*' => {
                token = LexerToken::Star;
            },
            '/' => {
                if self.peek(0) == '/'
                {
                    self.next();
                    while self.peek(0) != '\r' && self.peek(0) != '\n' && self.peek(0) != '\0'
                    {
                        self.next();
                    }
                    token = LexerToken::BadToken;
                    lexnext = true;
                }
                else
                if self.peek(0) == '*'
                {
                    self.next();
                    while self.peek(0) != '*' && self.peek(1) != '/'
                    {
                        self.next();
                    }
                    self.next();
                    self.next();
                    token = LexerToken::BadToken;
                    lexnext = true;
                }
                else
                {
                    token = LexerToken::Slash;
                }
            },
            ',' => {
                token = LexerToken::Comma;
            },
            ':' => {
                token = LexerToken::Colon;
            },
            '(' => {
                token = LexerToken::Openparenthesis;
            },
            ')' => {
                token = LexerToken::Closeparenthesis;
            },
            ';' => {
                token = LexerToken::Semicolon;
            },
            '%' => {
                token = LexerToken::Percent;
            },
            '|' => {
                if self.peek(0) == '|'
                {
                    self.next();
                    token = LexerToken::PipePipe;
                }
                else
                {
                    token = LexerToken::Pipe;
                }
            },
            '&' => {
                if self.peek(0) == '&'
                {
                    self.next();
                    token = LexerToken::AmpersandAmpersand;
                }
                else
                {
                    token = LexerToken::Ampersand;
                }
            },
            '=' => {
                if self.peek(0) == '='
                {
                    self.next();
                    token = LexerToken::EqualsEquals;
                }
                else
                if self.peek(0) == '>'
                {
                    self.next();
                    token = LexerToken::Arrow;
                }
                else
                {
                    token = LexerToken::Equals;
                }
            },
            '<' => {
                if self.peek(0) == '='
                {
                    self.next();
                    token = LexerToken::LessEquals;
                }
                else
                {
                    token = LexerToken::Less;
                }
            },
            '>' => {
                if self.peek(0) == '='
                {
                    self.next();
                    token = LexerToken::GreaterEquals;
                }
                else
                {
                    token = LexerToken::Greater;
                }
            },
            '!' => {
                if self.peek(0) == '='
                {
                    self.next();
                    token = LexerToken::BangEquals;
                }
                else
                {
                    token = LexerToken::Bang;
                }
            },
            '0'..='9' => {
                let mut is_float = false;
                while self.peek(0).is_numeric() || (self.peek(0) == '.' && !is_float)
                {
                    if self.peek(0) == '.'
                    {
                        is_float = true;
                    }
                    self.next();
                }
                token = LexerToken::Literal;
            },
            '"' => {
                while self.peek(0) != '"'
                {
                    self.next();
                }
                self.next();
                token = LexerToken::Literal;
            },
            'a'..='z' | 'A'..='Z' => {
                while self.peek(0).is_alphanumeric() || self.peek(0) == '_' || self.peek(0) == '.'
                {
                    self.next();
                }
                let text = self.input[start..self.position].to_string();
                if naming_util::get_keywords().contains(&text)
                {
                    println!("keyword found: {}", text);
                    token = LexerToken::Keyword;
                }
                else
                if text == "true" || text == "false"
                {
                    token = LexerToken::Literal;
                }
                else
                {
                    token = LexerToken::Identifier;
                }
            },
            '.' => {
                token = LexerToken::Dot;
            },
            _ =>  {
                token = LexerToken::BadToken;
            },
        };
        if lexnext
        {
            return self.lex_token();
        }
        let end = if self.position < self.input.chars().count()-1 {self.position} else {self.input.chars().count()-1};
        let text: String;
        if start < end
        {
            text = self.input[start..end].to_string();
        }
        else
        {
            text = self.input.chars().last().unwrap().to_string();
        }
        return Token::new(token, text, start, self.position-start);
    }
    fn next(&mut self) -> char {
        let ret = self.peek(0);
        self.position+=1;
        ret
    }
    fn peek(&self, offset: usize) -> char {
        let index = self.position + offset;
        if index >= self.input.len()
        {
            return '\0';
        }
        return self.input.chars().nth(index).unwrap();
    }
}
