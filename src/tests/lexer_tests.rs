pub(crate) const TEST_KEYWORD: i32 = 0x1;
pub(crate) const TEST_LITERAL: i32 = 0x2;
pub(crate) const TEST_IDENTIFIER: i32 = 0x4;
pub(crate) const TEST_SYMBOL: i32 = 0x8;
pub(crate) const TEST_BADTOKEN: i32 = 0x10;
pub(crate) const TEST_ALL: i32 = 0x1F;
#[cfg(test)]
mod lexer_tests
{
    use crate::test_utils;
    #[test]
    fn lex_int_literal_assignment()
    {
        let test_string = "test:int=1;";
        let generated_tokens = test_utils::generate_assignment_tokens("test".to_string(), "1".to_string(), "int");
        let lexed_tokens = test_utils::lex_string(test_string);
        assert_eq!(generated_tokens, lexed_tokens);
    }
    #[test]
    fn lex_string_literal_assignment()
    {
        let test_string = "test:string=\"test\";";
        let generated_tokens = test_utils::generate_assignment_tokens("test".to_string(), "\"test\"".to_string(), "string");
        let lexed_tokens = test_utils::lex_string(test_string);
        assert_eq!(generated_tokens, lexed_tokens);
    }
    #[test]
    fn lex_float_literal_assignment()
    {
        let test_string = "test:float=1.0;";
        let generated_tokens = test_utils::generate_assignment_tokens("test".to_string(), "1.0".to_string(), "float");
        let lexed_tokens = test_utils::lex_string(test_string);
        assert_eq!(generated_tokens, lexed_tokens);
    }
    #[test]
    fn lex_bool_literal_assignment()
    {
        let test_string = "test:bool=true;";
        let generated_tokens = test_utils::generate_assignment_tokens("test".to_string(), "true".to_string(), "bool");
        let lexed_tokens = test_utils::lex_string(test_string);
        assert_eq!(generated_tokens, lexed_tokens);
    }
    #[test]
    fn lex_keyword()
    {
        let mut map = test_utils::create_map();
        test_utils::insert_map(&mut map, super::TEST_KEYWORD);
        test_utils::test_map(map);
    }
    #[test]
    fn lex_literal()
    {
        let mut map = test_utils::create_map();
        test_utils::insert_map(&mut map, super::TEST_LITERAL);
        test_utils::test_map(map);
    }
    #[test]
    fn lex_identifier()
    {
        let mut map = test_utils::create_map();
        test_utils::insert_map(&mut map, super::TEST_IDENTIFIER);
        test_utils::test_map(map);
    }
    #[test]
    fn lex_symbol()
    {
        let mut map = test_utils::create_map();
        test_utils::insert_map(&mut map, super::TEST_SYMBOL);
        test_utils::test_map(map);
    }
    #[test]
    fn lex_badtoken()
    {
        let mut map = test_utils::create_map();
        test_utils::insert_map(&mut map, super::TEST_BADTOKEN);
        test_utils::test_map(map);
    }
    #[test]
    fn lex_badtoken_inverted()
    {
        let mut map = test_utils::create_map();
        test_utils::insert_inverted(&mut map, super::TEST_BADTOKEN);
        test_utils::test_map_inverted(map);
    }
}