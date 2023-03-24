#[cfg(test)]
mod assignment_tests
{
    use crate::test_utils;
    #[test]
    fn test_int_assignment()
    {
        let assignment = test_utils::generate_int_assignment("test".to_string(), "1".to_string());
        let int_assignment = test_utils::parse_int_assignment("test".to_string(), "1".to_string());
        if int_assignment.len() != 1
        {
            panic!("Int assignment length is not 1");
        }
        assert_eq!(int_assignment, assignment);
    }
}