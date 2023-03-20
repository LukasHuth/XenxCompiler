use std::collections::HashMap;
use super::{
    Datatype,
    Arguments,
    Statement,
};
pub fn generate_functions(funtions: &mut HashMap<String, (Datatype, Arguments, Vec::<Statement>)>)
{
    insert_print(funtions);
}
fn insert_print(funtions: &mut HashMap<String, (Datatype, Arguments, Vec::<Statement>)>)
{
    let datatype = Datatype::new(super::statement::StatementDatatype::Void, vec![], false);
    let mut args = Arguments::new(vec![]);
    args.push(Statement::new("".to_string(), super::statement::StatementType::Argument, super::statement::StatementDatatype::Int, vec![], false));
    funtions.insert("print".to_string(), (datatype, args, vec![]));
}
pub fn is_std_function(name: &String) -> bool
{
    match name.as_str()
    {
        "print" => true,
        _ => false,
    }
}