
#[cfg(test)]
mod tests
{
    use test_case::test_case;
    #[test_case("a: int = 6;" => g_r_s_f_d_a_i("int", "6", "a") ; "string init and decl")]
    #[test_case("a: bool = true;" => g_r_s_f_d_a_i("bool", "true", "a") ; "boolean initialization and declaration")]
    #[test_case("a: char = \"a\";" => g_r_s_f_d_a_i("char", "\"a\"", "a") ; "character initialization and declaration")]
    #[test_case("a: string = \"a\";" => g_r_s_f_d_a_i("string", "\"a\"", "a") ; "string initialization and declaration")]
    #[test_case("a: float = 6.6;" => g_r_s_f_d_a_i("float", "6.6", "a") ; "float initialization and declaration")]
    #[test_case("a: int;" => g_r_s_f_d("int", "a") ; "integer initialization")]
    #[test_case("a: bool;" => g_r_s_f_d("bool", "a") ; "boolean initialization")]
    #[test_case("a: char;" => g_r_s_f_d("char", "a") ; "character initialization")]
    #[test_case("a: string;" => g_r_s_f_d("string", "a") ; "string initialization")]
    #[test_case("a: float;" => g_r_s_f_d("float", "a") ; "float initialization")]
    #[test_case("a = 6;" => g_r_s_f_i("a", "6") ; "integer declaration")]
    #[test_case("a = true;" => (g_r_s_f_i("a", "true")) ; "boolean declaration")]
    #[test_case("a = \"a\";" => g_r_s_f_i("a", "\"a\"") ; "character declaration")]
    #[test_case("a = \"Hallo\";" => g_r_s_f_i("a", "\"Hallo\"") ; "string declaration")]
    #[test_case("a = 6.6;" => g_r_s_f_i("a", "6.6") ; "float declaration")]
    fn test(source: &str) -> String{
        let src = String::from(source);
        let mut _lexer = crate::lexer::Lexer::new(src);
        let mut parser = crate::parser::Parser::new(_lexer.lex());
        let p = parser.parse();
        let f = p.first().unwrap();
        f.to_string()
    }
    // generate result string for decl and init
    fn g_r_s_f_d_a_i(type_:&str, value: &str, name: &str) -> String
    {
        format!("AssignmentExpression: (type:{} value:{} name:VariableExpression: ({}))", type_, value, name)
    }
    // generate result string for decl
    fn g_r_s_f_d(type_:&str, name: &str) -> String
    {
        format!("AssignmentExpression: (type:{} value:0 name:VariableExpression: ({}))", type_, name)
    }
    // generate result string forr init
    fn g_r_s_f_i(name:&str, value: &str) -> String
    {
        format!("OverwriteVariableExpression: (name:{} value:{})", name, value)
    }
}