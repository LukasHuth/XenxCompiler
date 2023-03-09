use std::collections::HashMap;
use super::Datatype;

pub fn get_variable(name: String, variables: &HashMap<String, Datatype>) -> Datatype
{
    variables.get(&name).unwrap().clone()
}