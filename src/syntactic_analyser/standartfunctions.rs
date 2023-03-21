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
    for name in get_print_variants()
    {
        // let name = "std::print";
        let mut args = Vec::<Expression>::new();
        let type_name = name.replace("std::print_", "");
        args.push(Expression::new_arg_variable_expr("a_16515156161615".to_string(), type_name, 0));
        let new_func_dec_expr = FunctionDeclarationExpression::new(name.to_string(), "void".to_string(), args, vec![]);
        let datatype = Datatype::new(super::statement::StatementDatatype::Void, vec![], false);
        //let mut args = Arguments::new(vec![]);
        let args = super::util::get_parameters(new_func_dec_expr);
        funtions.insert(name.to_string(), (datatype, args, vec![]));
    }
}
pub fn is_std_function(name: &String) -> bool
{
    if get_print_variants().contains(name) || name == "std::print"
    {
        return true;
    }
    return false;
}
fn get_print_variants() -> Vec<String>
{
    vec!["std::print_string".to_string(),"std::print_int".to_string(), "std::print_bool".to_string()]
}
pub fn get_variants(name: &String) -> Vec<String>
{
    match name.as_str()
    {
        "std::print" => get_print_variants(),
        _ => vec![],
    }
}