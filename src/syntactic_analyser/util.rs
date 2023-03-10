use std::collections::HashMap;
use super::Datatype;

pub fn get_variable(name: String, variables: &HashMap<String, Datatype>) -> Datatype
{
    variables.get(&name).unwrap().clone()
}

use super::StatementDatatype;
use super::Expression;
#[allow(unreachable_patterns)]
pub fn test_variable_declaration_literal(datatype: Datatype, value: Expression, supress_output: bool) -> bool
{
    // println!("test for: {}", value.to_string());
    match datatype.datatype
    {
        StatementDatatype::Int => {
            if !value.is_integer_literal()
            {
                if !supress_output
                {
                    println!("variable is not an int");
                }
                return false;
            }
            return true;
        }
        StatementDatatype::Float => {
            if !value.is_float_literal()
            {
                if !supress_output
                {
                    println!("variable is not a float");
                }
                return false;
            }
            return true;
        }
        StatementDatatype::String => {
            if !value.is_string_literal()
            {
                if !supress_output
                {
                    println!("variable is not a string");
                }
                return false;
            }
            return true;
        }
        StatementDatatype::Bool => {
            if !value.is_boolean_literal()
            {
                if !supress_output
                {
                    println!("variable is not a bool");
                }
                return false;
            }
            return true;
        }
        StatementDatatype::Void => {
            if !supress_output
            {
                println!("variable is void");
            }
            return false;
        }
        StatementDatatype::Char => {
            if !value.is_char_literal()
            {
                if !supress_output
                {
                    println!("variable is not a char");
                }
                return false;
            }
            return true;
        }
        _ => {
            if !supress_output
            {
                println!("variable is not a valid datatype");
            }
            return false;
        }
    }
}

use super::Statement;
pub fn same_datatype(arg1: Statement, arg2: Datatype) -> bool {
    let arg1 = arg1.datatype.clone();
    if arg1.datatype != arg2.datatype {
        return false;
    }
    if arg1.is_array != arg2.is_array {
        return false;
    }
    if arg1.array_bounds != arg2.array_bounds {
        return false;
    }
    return true;
}

pub fn test_variable_declaration_variable(datatype: Datatype, value: Expression, supress_output: bool, variables: &HashMap<String, Datatype>) -> bool
{
    // println!("test for: {}", value.to_string());
    let variable = value.syntax.get_variable_expr();
    let name = variable.get_name(); 
    // println!("test for: {}", name);
    let variable = get_variable(name, &variables);
    // println!("variable: {} datatype: {}", variable, datatype);
    // println!("variable: {} datatype: {}", variable.is_array, datatype.is_array);
    // println!("variable: {:?} datatype: {:?}", variable.array_bounds, datatype.array_bounds);
    if datatype.datatype == variable.datatype && datatype.is_array == variable.is_array && datatype.array_bounds == variable.array_bounds
    {
        return true;
    }
    if !supress_output
    {
        println!("variable declaration is not the same type as the variable");
    }
    return false;
}

#[allow(dead_code)]
pub fn get_lit_datatype(string: String) -> Datatype
{
    let datatype: StatementDatatype;
    let datastring = string.clone();
    let datastring = datastring.to_lowercase();
    let datastring = datastring.trim();
    match string.parse::<i32>()
    {
        Ok(_) => datatype = StatementDatatype::Int,
        Err(_) => {
            match string.parse::<f32>()
            {
                Ok(_) => datatype = StatementDatatype::Float,
                Err(_) => {
                    if datastring == "true" || datastring == "false"
                    {
                        datatype = StatementDatatype::Bool;
                    }
                    else
                    {
                        datatype = StatementDatatype::String;
                    }
                }
            }
        },
    }
    let datatype = Datatype::new(datatype, vec![], false);
    return datatype;
}

pub fn remove_n_chars_from_behind(string: String, n: usize) -> String
{
    let mut string = string;
    for _ in 0..n
    {
        string.pop();
    }
    return string;
}

pub fn get_datatype_from_string(datastring: String) -> StatementDatatype
{
    let mut datatype = StatementDatatype::Void;
    let datastring = datastring.clone();
    let datastring = datastring.to_lowercase();
    let datastring = datastring.trim();
    if datastring == "int"
    {
        datatype = StatementDatatype::Int;
    }
    else
    if datastring == "float"
    {
        datatype = StatementDatatype::Float;
    }
    else
    if datastring == "string"
    {
        datatype = StatementDatatype::String;
    }
    else
    if datastring == "bool"
    {
        datatype = StatementDatatype::Bool;
    }
    else
    if datastring == "void"
    {
        datatype = StatementDatatype::Void;
    }
    return datatype;
}

pub fn get_line_of_position(context: String, position: usize) -> (usize, usize) {
    let mut start = 0;
    let mut line_ = 1;
    let mut pos = 0;
    while pos < context.chars().count()
    {
        if pos == position
        {
            return (line_, start-1);
        }
        let line = context.chars().nth(pos).unwrap();
        if line == '\n'
        {
            line_ += 1;
            start = 0;
        }
        start += 1;
        pos += 1;
    }
    return (line_, start);
}