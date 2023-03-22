const TEST_KEYWORD: i32 = 0x1;
const TEST_LITERAL: i32 = 0x2;
const TEST_IDENTIFIER: i32 = 0x4;
const TEST_SYMBOL: i32 = 0x8;
const TEST_BADTOKEN: i32 = 0x10;
const TEST_ALL: i32 = 0x1F;
pub mod test_util;
#[cfg(test)]
mod lexer_tests
{
    use crate::test::test_util;
    use crate::lexer::token::LexerToken;
    use crate::test::*;
    use std::collections::HashMap;
    #[test]
    fn lex_keyword()
    {
        let mut map = HashMap::<String, LexerToken>::new();
        test_util::insert_map(&mut map, TEST_KEYWORD);
        test_util::test_map(map);
    }
    #[test]
    fn lex_literal()
    {
        let mut map = HashMap::<String, LexerToken>::new();
        test_util::insert_map(&mut map, TEST_LITERAL);
        test_util::test_map(map);
    }
    #[test]
    fn lex_identifier()
    {
        let mut map = HashMap::<String, LexerToken>::new();
        test_util::insert_map(&mut map, TEST_IDENTIFIER);
        test_util::test_map(map);
    }
    #[test]
    fn lex_symbol()
    {
        let mut map = HashMap::<String, LexerToken>::new();
        test_util::insert_map(&mut map, TEST_SYMBOL);
        test_util::test_map(map);
    }
    #[test]
    fn lex_badtoken()
    {
        let mut map = HashMap::<String, LexerToken>::new();
        test_util::insert_map(&mut map, TEST_BADTOKEN);
        test_util::test_map(map);
    }
    #[test]
    fn lex_badtoken_inverted()
    {
        let mut map = HashMap::<String, LexerToken>::new();
        test_util::insert_inverted(&mut map, TEST_BADTOKEN);
        test_util::test_map_inverted(map);
    }
}