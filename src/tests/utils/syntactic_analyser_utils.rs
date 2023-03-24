use std::collections::HashMap;

use crate::{
    parser::expression::{
        Expression,
        FunctionDeclarationExpression
    },
    syntactic_analyser::{
        statement::{
            Statement,
            Datatype,
            StatementDatatype
        },
        SyntaticAnalyser,
        arguments::Arguments
    }
};

pub fn syntactic_analyser_get_body(statements: Vec<Expression>) -> Vec<Statement>
{
    let mut syntactic_analyser = SyntaticAnalyser::new(vec![], "".to_string(), false);
    let variables = generate_variables();
    let functiondatatype = generate_function_datatype();
    let args = generate_arguments();
    let ret = syntactic_analyser.get_body(statements, variables, functiondatatype, args, true);
    return ret;
}
fn generate_variables() -> HashMap<String, Datatype>
{
    #[allow(unused_mut)]
    let mut variables = HashMap::<String, Datatype>::new();
    // let datatype = StatementDatatype::Int;
    // let array_bounds = Vec::<i32>::new();
    // let datatype = Datatype::new(datatype, array_bounds.clone(), array_bounds.len() > 0);
    // variables.insert("test".to_string(), datatype);
    return variables;
}
fn generate_function_datatype() -> Datatype
{
    let datatype = StatementDatatype::Int;
    let array_bounds = Vec::<i32>::new();
    let datatype = Datatype::new(datatype, array_bounds.clone(), array_bounds.len() > 0);
    return datatype;
}
fn generate_arguments() -> Arguments
{
    let func_decleration_expression = FunctionDeclarationExpression::new("test".to_string(), "int".to_string(), vec![], vec![]);
    let args: Arguments = crate::syntactic_analyser::util::get_parameters(func_decleration_expression);
    return args;
}