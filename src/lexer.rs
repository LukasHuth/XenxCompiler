pub mod token;
use token::LexerToken;
use token::Token;
pub struct Lexer {
    input: String,
    position: usize,
}
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
        // println!("Found {}{}", current, self.peek(0));
        match current
        {
            ' ' | '\t' | '\r' | '\n' => self.lex_token(),
            '{' => {
                token = LexerToken::Openbrace;
                text = "{".to_string();
                return Token::new(token, text);
            },
            '}' => {
                token = LexerToken::Closebrace;
                text = "}".to_string();
                return Token::new(token, text);
            },
            '(' => {
                token = LexerToken::Openparenthesis;
                text = "(".to_string();
                return Token::new(token, text);
            },
            ')' => {
                token = LexerToken::Closeparenthesis;
                text = ")".to_string();
                return Token::new(token, text);
            },
            ';' => {
                token = LexerToken::Semicolon;
                text = ";".to_string();
                return Token::new(token, text);
            },
            '=' => {
                if self.peek(0) == '='
                {
                    self.next();
                    token = LexerToken::EqualsEquals;
                    text = "==".to_string();
                    return Token::new(token, text);
                }
                token = LexerToken::Equals;
                text = "=".to_string();
                return Token::new(token, text);
            },
            '<' => {
                if self.peek(0) == '='
                {
                    self.next();
                    token = LexerToken::LessEquals;
                    text = "<=".to_string();
                    return Token::new(token, text);
                }
                token = LexerToken::Less;
                text = "<".to_string();
                return Token::new(token, text);
            },
            '>' => {
                if self.peek(0) == '='
                {
                    self.next();
                    token = LexerToken::GreaterEquals;
                    text = ">=".to_string();
                    return Token::new(token, text);
                }
                token = LexerToken::Greater;
                text = ">".to_string();
                return Token::new(token, text);
            },
            '!' => {
                if self.peek(0) == '='
                {
                    self.next();
                    token = LexerToken::BangEquals;
                    text = "!=".to_string();
                    return Token::new(token, text);
                }
                token = LexerToken::Bang;
                text = "!".to_string();
                return Token::new(token, text);
            },
            '0'..='9' => {
                while self.peek(0).is_numeric()
                {
                    self.next();
                }
                token = LexerToken::IntegerLiteral;
                text = self.input[start..self.position].to_string();
                return Token::new(token, text);
            },
            'a'..='z' | 'A'..='Z' => {
                while self.peek(0).is_alphanumeric()
                {
                    self.next();
                }
                text = self.input[start..self.position].to_string();
                if text == "int"
                {
                    token = LexerToken::IntKeyword;
                }
                else if text == "return"
                {
                    token = LexerToken::ReturnKeyword;
                }
                else
                {
                    token = LexerToken::Identifier;
                }
                return Token::new(token, text);
            },
            _ =>  {
                token = LexerToken::BadToken;
                text = "".to_string();
                // self.input[start..self.position].to_string();
                return Token::new(token, text);
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