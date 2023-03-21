use std::collections::HashMap;
use crate::parser::expression::{FunctionDeclarationExpression, Expression};

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
    let name = "std::print";
    let mut args = Vec::<Expression>::new();
    args.push(Expression::new_arg_variable_expr("a_16515156161615".to_string(), "int".to_string(), 0));
    let new_func_dec_expr = FunctionDeclarationExpression::new(name.to_string(), "void".to_string(), args, vec![]);
    let datatype = Datatype::new(super::statement::StatementDatatype::Void, vec![], false);
    //let mut args = Arguments::new(vec![]);
    let args = super::util::get_parameters(new_func_dec_expr);
    funtions.insert(name.to_string(), (datatype, args, vec![]));
}
pub fn is_std_function(name: &String) -> bool
{
    match name.as_str()
    {
        "std::print" => true,
        _ => false,
    }
}