use super::{util,Datatype,Expression,LexerToken,StatementDatatype,Arguments,Statement};
use super::HashMap;
pub fn is_datatype(datatype: Datatype, value: Expression, supressed_output: bool, variables: &HashMap<String, Datatype>,
    functions: &HashMap<String, (Datatype, Arguments, Vec::<Statement>)>) -> bool
{
    return test_variable_declaration(datatype, value, supressed_output, &variables, &functions);
}
pub fn test_variable_declaration(datatype: Datatype, value: Expression, supress_output: bool, variables: &HashMap<String, Datatype>
    , functions: &HashMap<String, (Datatype, Arguments, Vec::<Statement>)>) -> bool
{
    if value.is_literal()
    {
        return util::test_variable_declaration_literal(datatype, value, supress_output);
    }
    else
    if value.is_variable()
    {
        return util::test_variable_declaration_variable(datatype, value, supress_output, &variables);
    }
    else
    if value.is_binary()
    {
        return test_variable_declaration_binary(datatype, value, supress_output, &variables, &functions);
    }
    if value.is_call()
    {
        return test_variable_declaration_call(datatype, value, supress_output, &variables, &functions);
    }
    if value.is_unary()
    {
        let unary = value.syntax.get_unary_expr();
        return test_variable_declaration(datatype, unary.get_operand(), supress_output, &variables, &functions);
    }
    else
    {
        if !supress_output
        {
            println!("variable declaration is not a literal or variable");
        }
        return false;
    }
}
fn test_variable_declaration_binary(datatype: Datatype, value: Expression, supress_output: bool, variables: &HashMap<String, Datatype>,
    functions: &HashMap<String, (Datatype, Arguments, Vec::<Statement>)>) -> bool
{
    let binary = value.syntax.get_binary_expr();
    let left = binary.get_left();
    let right = binary.get_right();
    let operator = binary.get_operator();
    if (operator.token == LexerToken::EqualsEquals || operator.token == LexerToken::BangEquals
        || operator.token == LexerToken::Less || operator.token == LexerToken::Greater
        || operator.token == LexerToken::LessEquals || operator.token == LexerToken::GreaterEquals
        || operator.token == LexerToken::AmpersandAmpersand || operator.token == LexerToken::PipePipe) 
        && datatype.datatype == StatementDatatype::Bool
    {
        let boolstate = Datatype::new(StatementDatatype::Bool, vec![], false);
        let stringstate = Datatype::new(StatementDatatype::String, vec![], false);
        let intstate = Datatype::new(StatementDatatype::Int, vec![], false);
        let charstate = Datatype::new(StatementDatatype::Char, vec![], false);
        let floatstate = Datatype::new(StatementDatatype::Float, vec![], false);
        if is_datatype(boolstate.clone(), left.clone(), supress_output, &variables, &functions) && is_datatype(boolstate.clone(), right.clone(), supress_output, &variables, &functions)
        {
            return true;
        }
        if operator.token == LexerToken::AmpersandAmpersand || operator.token == LexerToken::PipePipe
        {
            if !supress_output
            {
                println!("cannot use && or || on other that bool");
            }
            return false;
        }
        if util::test_datatype(stringstate.clone(), left.clone(),right.clone(), supress_output, &variables, &functions)
        {
            return true;
        }
        if util::test_datatype(intstate.clone(), left.clone(),right.clone(), supress_output, &variables, &functions)
        {
            return true;
        }
        if util::test_datatype(charstate.clone(), left.clone(),right.clone(), supress_output, &variables, &functions)
        {
            return true;
        }
        if util::test_datatype(floatstate.clone(), left.clone(),right.clone(), supress_output, &variables, &functions)
        {
            return true;
        }
    }
    else
    {
        let boolstate = Datatype::new(StatementDatatype::Bool, vec![], false);
        let stringstate = Datatype::new(StatementDatatype::String, vec![], false);
        let intstate = Datatype::new(StatementDatatype::Int, vec![], false);
        let charstate = Datatype::new(StatementDatatype::Char, vec![], false);
        let floatstate = Datatype::new(StatementDatatype::Float, vec![], false);
        if util::test_datatype(boolstate.clone(), left.clone(), right.clone(), supress_output, &variables, &functions)
        {
            if !supress_output
            {
                println!("cannot use binary operators except logical equals and not equals on bools");
            }
            return false;
        }
        if util::test_datatype(stringstate.clone(), left.clone(), right.clone(), supress_output, &variables, &functions)
        {
            if datatype.datatype == StatementDatatype::String
            {
                return true;
            }
            else
            {
                if !supress_output
                {
                    println!("cannot use binary operators on strings except logical equals and not equals");
                }
                return false;
            }
        }
        if util::test_datatype(intstate.clone(), left.clone(), right.clone(), supress_output, &variables, &functions)
        {
            if datatype.datatype == StatementDatatype::Int
            {
                return true;
            }
            else
            {
                if !supress_output
                {
                    println!("cannot use binary operators on ints except logical equals and not equals");
                }
                return false;
            }
        }
        if util::test_datatype(charstate.clone(), left.clone(), right.clone(), supress_output, &variables, &functions)
        {
            if datatype.datatype == StatementDatatype::Char
            {
                return true;
            }
            else
            {
                if !supress_output
                {
                    println!("cannot use binary operators on chars except logical equals and not equals");
                }
                return false;
            }
        }
        if util::test_datatype(floatstate.clone(), left.clone(), right.clone(), supress_output, &variables, &functions)
        {
            if datatype.datatype == StatementDatatype::Float
            {
                return true;
            }
            else
            {
                if !supress_output
                {
                    println!("cannot use binary operators on floats except logical equals and not equals");
                }
                return false;
            }
        }
    }
    return false;
}

fn test_variable_declaration_call(datatype: Datatype, value: Expression, supress_output: bool, variables: &HashMap<String, Datatype>
        , functions: &HashMap<String, (Datatype, Arguments, Vec::<Statement>)>) -> bool {
    let call = value.clone().syntax.get_call_expr();
    let name = call.get_name();
    let args = call.get_args();
    let function = util::get_function(name, value.clone().get_position(), &functions);
    // println!("function: {}", function.0.to_string());
    // println!("datatype: {}", datatype.to_string());
    if args.len() != function.1.len() {
        if !supress_output {
            println!("function call does not have the same amount of arguments as the function");
        }
        return false;
    }
    for i in 0..args.len() {
        let arg = args[i].clone();
        let arg2 = function.1[i].clone();
        let arg2 = arg2.datatype;
        if !test_variable_declaration(arg2, arg, false, &variables, &functions) {
            if !supress_output {
                println!("function call does not have the same datatype as the function");
            }
            return false;
        }
    }
    if datatype.datatype != function.0.datatype {
        if !supress_output {
            println!("function return type does not match variable type");
        }
        return false;
    }
    return true;
}