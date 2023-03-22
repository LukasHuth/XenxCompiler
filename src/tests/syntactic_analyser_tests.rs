#[cfg(test)]
mod assignment_tests {
    use crate::test_utils;
    #[test]
    fn test_int_assignment() {
        let statement = test_utils::generate_int_assignment_statement("test".to_string(), "1".to_string());
        let statements = test_utils::generate_int_assignment("test".to_string(), "1".to_string());
        let body = test_utils::syntactic_analyser_get_body(statements);
        if body.len() != 1
        {
            panic!("Body length is not 1");
        }
        let body_statement = &body[0];
        if body_statement.name != statement.name
        {
            panic!("Body statement name is not equal to statement name");
        }
        assert_eq!(body_statement, &statement);
        println!("Body: {:?}", body);
    }
}
