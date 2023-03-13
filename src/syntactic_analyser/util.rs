use std::collections::HashMap;
use super::Datatype;

pub fn get_variable(name: String, variables: &HashMap<String, Datatype>) -> Datatype
{
    if variables.keys().find(|&x| *x == name).is_none()
    {
        panic!("variable {} does not exist", name);
    }
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

use super::Arguments;

pub fn generate_binary(expression: Expression, vars: &HashMap<String, Datatype>, functions: &HashMap<String, (Datatype, Arguments, Vec::<Statement>)>) -> Statement {
    if expression.is_literal()
    {
        return generate_literal(expression);
    }
    if expression.is_variable()
    {
        return generate_variable(expression, vars);
    }
    if expression.is_call()
    {
        return generate_function_call(expression, vars, functions);
    }
    if expression.is_unary()
    {
        return generate_unary(expression, vars, functions);
    }
    let binary = expression.syntax.get_binary_expr();
    let left = binary.get_left();
    let right = binary.get_right();
    let operator = binary.get_operator();
    let left = generate_binary(left, vars, functions);
    let right = generate_binary(right, vars, functions);
    let left_type = left.datatype.clone();
    let right_type = right.datatype.clone();
    let left_type = left_type.datatype;
    let right_type = right_type.datatype;
    let operator = get_op_by_token(operator.token);
    let datatype = get_datatype_by_datatype_and_operator(left_type, right_type, operator.clone());
    return Statement::new_binary(left, right, operator, Datatype::new(datatype, vec![], false));
}

use super::StatementType;

fn generate_variable(expression: Expression, vars: &HashMap<String, Datatype>) -> Statement {
    let variable = expression.syntax.get_variable_expr();
    let name = variable.get_name();
    let datatype = get_variable(name.clone(), &vars);
    return Statement::new(name.clone(), StatementType::Variable, datatype.datatype, datatype.array_bounds, datatype.is_array);
}

fn generate_literal(expr: Expression) -> Statement {
    let value: String;
    let datatype: StatementDatatype;
    if expr.is_integer_literal()
    {
        value = expr.syntax.get_integer_literal().to_string();
        datatype = StatementDatatype::Int;
    }
    else if expr.is_float_literal()
    {
        value = expr.syntax.get_float_literal().to_string();
        datatype = StatementDatatype::Float;
    }
    else if expr.is_string_literal()
    {
        value = expr.syntax.get_string_literal().to_string();
        datatype = StatementDatatype::String;
    }
    else if expr.is_boolean_literal()
    {
        value = expr.syntax.get_boolean_literal().to_string();
        datatype = StatementDatatype::Bool;
    }
    else
    {
        panic!("Invalid literal (not int, float, string or bool)");
    }
    return Statement::new(value, StatementType::Literal, datatype, vec![], false);
}

fn generate_unary(expr: Expression, vars: &HashMap<String, Datatype>, functions: &HashMap<String, (Datatype, Arguments, Vec::<Statement>)>) -> Statement {
    let unary = expr.syntax.get_unary_expr();
    let operator = unary.get_operator();
    let operator = get_op_by_token(operator.token);
    let operand = unary.get_operand();
    let operand = generate_binary(operand, vars, functions);
    let datatype = get_datatype_by_datatype_and_operator(operand.datatype.datatype, StatementDatatype::Void, operator.clone());
    return Statement::new_unary(operand, operator, Datatype::new(datatype, vec![], false));
}

fn generate_function_call(expression: Expression, vars: &HashMap<String, Datatype>, functions: &HashMap<String, (Datatype, Arguments, Vec::<Statement>)>) -> Statement {
    let call = expression.syntax.get_call_expr();
    let name = call.get_name();
    let mut statements = Vec::<Statement>::new();
    for arg in call.get_args()
    {
        statements.push(generate_binary(arg.clone(), &vars, &functions));
    }
    let data = functions.get(&name).unwrap();
    let call_state = Statement::new_call(name, statements, data.0.clone());
    return call_state;
}

fn get_datatype_by_datatype_and_operator(left: StatementDatatype, right: StatementDatatype, operator: String) -> StatementDatatype {
    if left == StatementDatatype::Int && right == StatementDatatype::Int
    {
        if operator == "+" || operator == "-" || operator == "*" || operator == "/" || operator == "%"
        {
            return StatementDatatype::Int;
        }
        if operator == "==" || operator == "!=" || operator == ">" || operator == "<" || operator == ">=" || operator == "<="
        {
            return StatementDatatype::Bool;
        }
    }
    if right == StatementDatatype::Void
    {
        if left == StatementDatatype::Int
        {
            if operator == "-" || operator == "!"
            {
                return StatementDatatype::Int;
            }
        }
        if left == StatementDatatype::Bool
        {
            if operator == "!"
            {
                return StatementDatatype::Bool;
            }
        }
    }
    panic!("Datatype not implemented yet");
}

use super::LexerToken;
pub fn get_op_by_token(token : LexerToken) -> String
{
    return match token {
        LexerToken::Plus => String::from("+"),
        LexerToken::Minus => String::from("-"),
        LexerToken::Star => String::from("*"),
        LexerToken::Slash => String::from("/"),
        // LexerToken::Percent => String::from("%"), // TODO: Implement
        LexerToken::EqualsEquals => String::from("=="),
        LexerToken::BangEquals => String::from("!="),
        LexerToken::Less => String::from("<"),
        LexerToken::LessEquals => String::from("<="),
        LexerToken::Greater => String::from(">"),
        LexerToken::GreaterEquals => String::from(">="),
        LexerToken::AmpersandAmpersand => String::from("&&"),
        LexerToken::PipePipe => String::from("||"),
        LexerToken::Ampersand => String::from("&"),
        LexerToken::Pipe => String::from("|"),
        _ => panic!("Token is not an operator"),
    }
}
pub fn get_function(name: String, pos: usize, functions: &HashMap<String, (Datatype, Arguments, Vec::<Statement>)>) -> (Datatype, Vec<Statement>) {
    let funcs = functions.keys().map(|e| e.to_string() ).collect::<Vec<String>>();
    if !funcs.contains(&name) {
        // println!("pos: {}", pos);
        panic!("function {} does not exist", name);
    }
    let data = functions.get(&name).unwrap().clone();
    let data = (data.0.clone(), data.1.arguments.clone());
    return data;
}

pub fn test_datatype(datatype: Datatype, left: Expression, right: Expression, supress_output: bool, variables: &&HashMap<String, Datatype>, functions: &&HashMap<String, (Datatype, Arguments, Vec<Statement>)>) -> bool {
    let first = super::typetest::is_datatype(datatype.clone(), left, supress_output, &variables, &functions);
    let second = super::typetest::is_datatype(datatype.clone(), right, supress_output, &variables, &functions);
    return first && second;
}