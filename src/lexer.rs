pub mod token;
use token::{
    LexerToken,
    Token,
};
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
    pub fn lex_token(&mut self) -> Token
    {
        let start = self.position;
        let current = self.next();
        let token: LexerToken;
        let text: String;
        match current
        {
            ' ' | '\t' | '\r' | '\n' => self.lex_token(),
            '{' => {
                token = LexerToken::Openbrace;
                text = "{".to_string();
                return Token::new(token, text, start, self.position-start);
            },
            '}' => {
                token = LexerToken::Closebrace;
                text = "}".to_string();
                return Token::new(token, text, start, self.position-start);
            },
            '[' => {
                token = LexerToken::OpenSquareBracket;
                text = "[".to_string();
                return Token::new(token, text, start, self.position-start);
            },
            ']' => {
                token = LexerToken::CloseSquareBracket;
                text = "]".to_string();
                return Token::new(token, text, start, self.position-start);
            },
            '+' => {
                token = LexerToken::Plus;
                text = "+".to_string();
                return Token::new(token, text, start, self.position-start);
            },
            '-' => {
                token = LexerToken::Minus;
                text = "-".to_string();
                return Token::new(token, text, start, self.position-start);
            },
            '*' => {
                token = LexerToken::Star;
                text = "*".to_string();
                return Token::new(token, text, start, self.position-start);
            },
            '/' => {
                if self.peek(0) == '/'
                {
                    self.next();
                    while self.peek(0) != '\r' && self.peek(0) != '\n' && self.peek(0) != '\0'
                    {
                        self.next();
                    }
                    return self.lex_token();
                }
                if self.peek(0) == '*'
                {
                    self.next();
                    while self.peek(0) != '*' && self.peek(1) != '/'
                    {
                        self.next();
                    }
                    self.next();
                    self.next();
                    return self.lex_token();
                }
                token = LexerToken::Slash;
                text = "/".to_string();
                return Token::new(token, text, start, self.position-start);
            },
            ',' => {
                token = LexerToken::Comma;
                text = ",".to_string();
                return Token::new(token, text, start, self.position-start);
            },
            ':' => {
                token = LexerToken::Colon;
                text = ":".to_string();
                return Token::new(token, text, start, self.position-start);
            },
            '(' => {
                token = LexerToken::Openparenthesis;
                text = "(".to_string();
                return Token::new(token, text, start, self.position-start);
            },
            ')' => {
                token = LexerToken::Closeparenthesis;
                text = ")".to_string();
                return Token::new(token, text, start, self.position-start);
            },
            ';' => {
                token = LexerToken::Semicolon;
                text = ";".to_string();
                return Token::new(token, text, start, self.position-start);
            },
            '%' => {
                token = LexerToken::Percent;
                text = "%".to_string();
                return Token::new(token, text, start, self.position-start);
            },
            '|' => {
                if self.peek(0) == '|'
                {
                    self.next();
                    token = LexerToken::PipePipe;
                    text = "||".to_string();
                    return Token::new(token, text, start, self.position-start);
                }
                token = LexerToken::Pipe;
                text = "|".to_string();
                return Token::new(token, text, start, self.position-start);
            },
            '&' => {
                if self.peek(0) == '&'
                {
                    self.next();
                    token = LexerToken::AmpersandAmpersand;
                    text = "&&".to_string();
                    return Token::new(token, text, start, self.position-start);
                }
                token = LexerToken::Ampersand;
                text = "&".to_string();
                return Token::new(token, text, start, self.position-start);
            },
            '=' => {
                if self.peek(0) == '='
                {
                    self.next();
                    token = LexerToken::EqualsEquals;
                    text = "==".to_string();
                    return Token::new(token, text, start, self.position-start);
                }
                else
                if self.peek(0) == '>'
                {
                    self.next();
                    token = LexerToken::Arrow;
                    text = "=>".to_string();
                    return Token::new(token, text, start, self.position-start);
                }
                token = LexerToken::Equals;
                text = "=".to_string();
                return Token::new(token, text, start, self.position-start);
            },
            '<' => {
                if self.peek(0) == '='
                {
                    self.next();
                    token = LexerToken::LessEquals;
                    text = "<=".to_string();
                    return Token::new(token, text, start, self.position-start);
                }
                token = LexerToken::Less;
                text = "<".to_string();
                return Token::new(token, text, start, self.position-start);
            },
            '>' => {
                if self.peek(0) == '='
                {
                    self.next();
                    token = LexerToken::GreaterEquals;
                    text = ">=".to_string();
                    return Token::new(token, text, start, self.position-start);
                }
                token = LexerToken::Greater;
                text = ">".to_string();
                return Token::new(token, text, start, self.position-start);
            },
            '!' => {
                if self.peek(0) == '='
                {
                    self.next();
                    token = LexerToken::BangEquals;
                    text = "!=".to_string();
                    return Token::new(token, text, start, self.position-start);
                }
                token = LexerToken::Bang;
                text = "!".to_string();
                return Token::new(token, text, start, self.position-start);
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
                text = self.input[start..self.position].to_string();
                return Token::new(token, text, start, self.position-start);
            },
            '"' => {
                while self.peek(0) != '"'
                {
                    self.next();
                }
                self.next();
                token = LexerToken::Literal;
                text = self.input[start..self.position].to_string();
                return Token::new(token, text, start, self.position-start);
            },
            'a'..='z' | 'A'..='Z' => {
                while self.peek(0).is_alphanumeric()
                {
                    self.next();
                }
                text = self.input[start..self.position].to_string();
                if text == "return" || text == "float" || text == "int" || text == "func" || text == "bool" || text == "string"
                    || text == "if" || text == "else" || text == "char" || text == "while" || text == "for" || text == "break"
                    || text == "continue" || text == "struct" || text == "import" || text == "as" || text == "null"
                {
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
                return Token::new(token, text, start, self.position-start);
            },
            '.' => {
                token = LexerToken::Dot;
                text = ".".to_string();
                return Token::new(token, text, start, self.position-start);
            },
            _ =>  {
                token = LexerToken::BadToken;
                text = "".to_string();
                return Token::new(token, text, start, self.position-start);
            },
        }
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