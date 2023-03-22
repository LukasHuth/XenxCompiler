
use crate::lexer;
use crate::test::*;
use lexer::token::LexerToken;
use std::collections::HashMap;
pub fn test_string(str: &str, expected: LexerToken)
{
    let mut lexer = lexer::Lexer::new(str.to_string());
    let tokens = lexer.lex();
    if tokens.len() > 1
    {
        panic!("More than one token returned");
    }
    assert_eq!(tokens[0].token, expected);
}
pub fn test_string_inverted(str: &str, expected: LexerToken)
{
    let mut lexer = lexer::Lexer::new(str.to_string());
    let tokens = lexer.lex();
    if tokens.len() > 1
    {
        panic!("More than one token returned");
    }
    assert_ne!(tokens[0].token, expected);
}
pub fn insert_map(map: &mut HashMap<String, LexerToken>, test_type: i32)
{
    if test_type^TEST_LITERAL == test_type
    {
        insert_literal(map);
    }
    if test_type^TEST_KEYWORD == test_type
    {
        insert_keyword(map);
    }
    if test_type^TEST_IDENTIFIER == test_type
    {
        map.insert("a".to_string(), LexerToken::Identifier);
        insert_identifier(map);
    }
    if test_type^TEST_SYMBOL == test_type
    {
        insert_symbols(map);
    }
    if test_type^TEST_BADTOKEN == test_type
    {
        insert_bad_token(map);
    }
}

fn insert_literal(map: &mut HashMap<String, LexerToken>) {
    map.insert("true".to_string(), LexerToken::Literal);
    map.insert("false".to_string(), LexerToken::Literal);
    map.insert("5".to_string(), LexerToken::Literal);
    map.insert("5.5".to_string(), LexerToken::Literal);
    map.insert("\"test\"".to_string(), LexerToken::Literal);
}

fn insert_keyword(map: &mut HashMap<String, LexerToken>) {
    map.insert("int".to_string(), LexerToken::Keyword);
    map.insert("float".to_string(), LexerToken::Keyword);
    map.insert("string".to_string(), LexerToken::Keyword);
    map.insert("bool".to_string(), LexerToken::Keyword);
    map.insert("if".to_string(), LexerToken::Keyword);
    map.insert("else".to_string(), LexerToken::Keyword);
    map.insert("while".to_string(), LexerToken::Keyword);
    map.insert("for".to_string(), LexerToken::Keyword);
    map.insert("return".to_string(), LexerToken::Keyword);
    map.insert("import".to_string(), LexerToken::Keyword);
    map.insert("func".to_string(), LexerToken::Keyword);
}

fn insert_identifier(map: &mut HashMap<String, LexerToken>) {
    map.insert("a5".to_string(), LexerToken::Identifier);
    map.insert("a_5".to_string(), LexerToken::Identifier);
    map.insert("a5_".to_string(), LexerToken::Identifier);
    map.insert("a5_5".to_string(), LexerToken::Identifier);
}

fn insert_symbols(map: &mut HashMap<String, LexerToken>) {
    map.insert(":".to_string(), LexerToken::Colon);
    map.insert("=".to_string(), LexerToken::Equals);
    map.insert("+".to_string(), LexerToken::Plus);
    map.insert("-".to_string(), LexerToken::Minus);
    map.insert("*".to_string(), LexerToken::Star);
    map.insert("/".to_string(), LexerToken::Slash);
    map.insert("(".to_string(), LexerToken::Openparenthesis);
    map.insert(")".to_string(), LexerToken::Closeparenthesis);
    map.insert("{".to_string(), LexerToken::Openbrace);
    map.insert("}".to_string(), LexerToken::Closebrace);
    map.insert("[".to_string(), LexerToken::OpenSquareBracket);
    map.insert("]".to_string(), LexerToken::CloseSquareBracket);
    map.insert(";".to_string(), LexerToken::Semicolon);
    map.insert(",".to_string(), LexerToken::Comma);
    map.insert(">".to_string(), LexerToken::Greater);
    map.insert("<".to_string(), LexerToken::Less);
    map.insert("==".to_string(), LexerToken::EqualsEquals);
    map.insert("!=".to_string(), LexerToken::BangEquals);
    map.insert(">=".to_string(), LexerToken::LessEquals);
    map.insert("<=".to_string(), LexerToken::LessEquals);
    map.insert("&&".to_string(), LexerToken::AmpersandAmpersand);
    map.insert("||".to_string(), LexerToken::PipePipe);
    map.insert("!".to_string(), LexerToken::Bang);
    map.insert("|".to_string(), LexerToken::Pipe);
    map.insert("&".to_string(), LexerToken::Ampersand);
    map.insert("%".to_string(), LexerToken::Percent);
    map.insert("^".to_string(), LexerToken::Caret);
    map.insert("\0".to_string(), LexerToken::EOF);
    map.insert("=>".to_string(), LexerToken::Arrow);
    map.insert(".".to_string(), LexerToken::Dot);
}

fn insert_bad_token(map: &mut HashMap<String, LexerToken>) {
    let used_symbols = vec!["+", "-", "*", "/", "(", ")", "{", "}", "[", "]", ";", ",", ">", "<", "!", "|", "&", "%", "^", "\0", "."];
    for i in 0..255
    {
        let str = String::from(i as u8 as char);
        if !used_symbols.contains(&str.as_str())
        {
            map.insert(str, LexerToken::BadToken);
        }
    }
}
pub fn test_map(map: HashMap<String, LexerToken>) {
    for keys in map
    {
        test_util::test_string(&keys.0, keys.1);
    }
}
pub fn test_map_inverted(map: HashMap<String, LexerToken>) {
    for keys in map
    {
        test_util::test_string_inverted(&keys.0, keys.1);
    }
}
pub fn insert_inverted(map: &mut HashMap<String, LexerToken>, test_type: i32)
{
    let test_type = test_type|TEST_ALL;
    if test_type^TEST_LITERAL == test_type
    {
        insert_literal_inverted(map);
    }
    if test_type^TEST_KEYWORD == test_type
    {
        insert_keyword_inverted(map);
    }
    if test_type^TEST_IDENTIFIER == test_type
    {
        insert_identifier_inverted(map);
    }
    if test_type^TEST_SYMBOL == test_type
    {
        insert_symbols_inverted(map);
    }
    if test_type^TEST_BADTOKEN == test_type
    {
        insert_bad_token_inverted(map);
    }
}
fn insert_literal_inverted(map: &mut HashMap<String, LexerToken>) {
    let mut new_map = HashMap::<String, LexerToken>::new();
    insert_literal(&mut new_map);
    for keys in new_map
    {
        map.insert(keys.0, LexerToken::BadToken);
    }
}
fn insert_keyword_inverted(map: &mut HashMap<String, LexerToken>) {
    let mut new_map = HashMap::<String, LexerToken>::new();
    insert_keyword(&mut new_map);
    for keys in new_map
    {
        map.insert(keys.0, LexerToken::BadToken);
    }
}
fn insert_identifier_inverted(map: &mut HashMap<String, LexerToken>) {
    let mut new_map = HashMap::<String, LexerToken>::new();
    insert_identifier(&mut new_map);
    for keys in new_map
    {
        map.insert(keys.0, LexerToken::BadToken);
    }
}
fn insert_symbols_inverted(map: &mut HashMap<String, LexerToken>) {
    let mut new_map = HashMap::<String, LexerToken>::new();
    insert_symbols(&mut new_map);
    for keys in new_map
    {
        map.insert(keys.0, LexerToken::BadToken);
    }
}
fn insert_bad_token_inverted(map: &mut HashMap<String, LexerToken>) {
    let mut new_map = HashMap::<String, LexerToken>::new();
    insert_bad_token(&mut new_map);
    for keys in new_map
    {
        map.insert(keys.0, LexerToken::Keyword);
    }
}