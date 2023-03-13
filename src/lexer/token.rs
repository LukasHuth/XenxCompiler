use std::fmt;

#[derive(Clone)]
pub struct Token {
    pub token: LexerToken,
    pub text: String,
    pub pos: usize,
    pub length: usize,
}
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum LexerToken {
    Openbrace,
    Closebrace,
    Openparenthesis,
    Closeparenthesis,
    Semicolon,
    Identifier,
    Literal,
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
    Colon,
    Comma,
    Keyword,
    Arrow,
    Plus,
    Minus,
    Star,
    Slash,
    Pipe,
    PipePipe,
    Ampersand,
    AmpersandAmpersand,
    OpenSquareBracket,
    CloseSquareBracket,
}
impl Token {
    pub fn new(token: LexerToken, text: String, pos: usize, length: usize) -> Token {
        Token {
            token,
            text,
            pos,
            length,
        }
    }
    pub fn precedence(&self) -> i32 {
        if self.token == LexerToken::PipePipe || self.token == LexerToken::AmpersandAmpersand {
            return 1;
        }
        if self.token == LexerToken::EqualsEquals
            || self.token == LexerToken::BangEquals
            || self.token == LexerToken::Less
            || self.token == LexerToken::LessEquals
            || self.token == LexerToken::Greater
            || self.token == LexerToken::GreaterEquals
        {
            return 2;
        }
        if self.token == LexerToken::Plus || self.token == LexerToken::Minus {
            return 3;
        }
        if self.token == LexerToken::Star || self.token == LexerToken::Slash {
            return 4;
        }
        return 0;
    }
    pub fn is_data_type(&self) -> bool {
        if self.token != LexerToken::Keyword {
            return false;
        }
        return self.text == "int"
            || self.text == "string"
            || self.text == "bool"
            || self.text == "float"
            || self.text == "char";
    }
    /* #region Literal typecheck */
    pub fn is_string(&self) -> bool {
        if self.token != LexerToken::Literal {
            return false;
        }
        return self.text.starts_with("\"") && self.text.ends_with("\"");
    }
    pub fn is_boolean(&self) -> bool {
        if self.token != LexerToken::Literal {
            return false;
        }
        return self.text == "true" || self.text == "false";
    }
    pub fn is_integer(&self) -> bool {
        if self.token != LexerToken::Literal {
            return false;
        }
        return self.text.parse::<i32>().is_ok();
    }
    pub fn is_float(&self) -> bool {
        if self.token != LexerToken::Literal {
            return false;
        }
        return self.text.parse::<f32>().is_ok();
    }
    /* #endregion */
    #[allow(dead_code)]
    fn token_to_string(&self) -> String {
        match self.token {
            LexerToken::Openbrace => "Openbrace".to_string(),
            LexerToken::Closebrace => "Closebrace".to_string(),
            LexerToken::Openparenthesis => "Openparenthesis".to_string(),
            LexerToken::Closeparenthesis => "Closeparenthesis".to_string(),
            LexerToken::Semicolon => "Semicolon".to_string(),
            LexerToken::Keyword => "Keyword".to_string(),
            LexerToken::Identifier => "Identifier".to_string(),
            LexerToken::Literal => "Literal".to_string(),
            LexerToken::EOF => "EOF".to_string(),
            LexerToken::Colon => "Colon".to_string(),
            LexerToken::Comma => "Comma".to_string(),
            LexerToken::Less => "Less".to_string(),
            LexerToken::LessEquals => "LessEquals".to_string(),
            LexerToken::Greater => "Greater".to_string(),
            LexerToken::GreaterEquals => "GreaterEquals".to_string(),
            LexerToken::Equals => "Equals".to_string(),
            LexerToken::EqualsEquals => "EqualsEquals".to_string(),
            LexerToken::Bang => "Bang".to_string(),
            LexerToken::BangEquals => "BangEquals".to_string(),
            LexerToken::Arrow => "Arrow".to_string(),
            LexerToken::Plus => "Plus".to_string(),
            LexerToken::Minus => "Minus".to_string(),
            LexerToken::Star => "Star".to_string(),
            LexerToken::Slash => "Slash".to_string(),
            LexerToken::Pipe => "Pipe".to_string(),
            LexerToken::PipePipe => "PipePipe".to_string(),
            LexerToken::Ampersand => "Ampersand".to_string(),
            LexerToken::AmpersandAmpersand => "AmpersandAmpersand".to_string(),
            LexerToken::OpenSquareBracket => "OpenSquareBracket".to_string(),
            LexerToken::CloseSquareBracket => "CloseSquareBracket".to_string(),
            _ => "BadToken".to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        if self.token == LexerToken::Identifier || self.token == LexerToken::Literal {
            return "<Token: ".to_string()
                + self.token_to_string().as_str()
                + " Text: "
                + &self.text
                + ">";
        }
        return "<Token: ".to_string() + "" + self.token_to_string().as_str() + ">";
    }
    pub fn clone(&self) -> Token {
        Token {
            token: self.token,
            text: self.text.clone(),
            pos: self.pos.clone(),
            length: self.length.clone(),
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
            LexerToken::Keyword => write!(f, "Keyword"),
            LexerToken::Identifier => write!(f, "Identifier"),
            LexerToken::Literal => write!(f, "Literal"),
            LexerToken::EOF => write!(f, "EOF"),
            LexerToken::Colon => write!(f, "Colon"),
            LexerToken::Comma => write!(f, "Comma"),
            LexerToken::Less => write!(f, "Less"),
            LexerToken::LessEquals => write!(f, "LessEquals"),
            LexerToken::Greater => write!(f, "Greater"),
            LexerToken::GreaterEquals => write!(f, "GreaterEquals"),
            LexerToken::Equals => write!(f, "Equals"),
            LexerToken::EqualsEquals => write!(f, "EqualsEquals"),
            LexerToken::Bang => write!(f, "Bang"),
            LexerToken::BangEquals => write!(f, "BangEquals"),
            LexerToken::Arrow => write!(f, "Arrow"),
            _ => write!(f, "BadToken"),
        }
    }
}
