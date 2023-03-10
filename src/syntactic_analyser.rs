use std::collections::HashMap;
pub mod statement;
pub mod arguments;
use statement::Statement;
use statement::StatementType;
use statement::StatementDatatype;

mod util;

use arguments::Arguments;
use statement::Datatype;

use super::lexer::token::LexerToken;
use super::parser::expression::Expression;

pub struct SyntaticAnalyser {
    pub statements: Vec<Expression>,
    pos: usize,
    functions: HashMap<String, (Datatype, Arguments, Vec::<Statement>)>,
    context: String,
}
impl SyntaticAnalyser {
    pub fn new(statements: Vec<Expression>, context: String) -> SyntaticAnalyser {
        // println!("start syntactic analyser");
        SyntaticAnalyser {
            statements: statements,
            pos: 0,
            functions: HashMap::<String, (Datatype, Arguments, Vec::<Statement>)>::new(),
            context: context,
        }
    }
    pub fn analyse(&mut self) -> Vec<Statement>
    {
        // println!("statements: {}", self.statements.len());
        let mut statements = Vec::<Statement>::new();
        while self.pos < self.statements.len()
        {
            let element = self.statements.get(self.pos).unwrap();
            if !element.is_function_declaration()
            {
                println!("first element is not a function declaration");
                return vec![];
            }
            let function_declaration_expr = element.syntax.get_function_declaration_expr();
            let name = function_declaration_expr.get_name();
            let datatype = self.get_datatype(function_declaration_expr.get_type());
            let parameters = self.get_parameters(function_declaration_expr);
            let mut function = Statement::new_datatype(name.clone(), StatementType::Function, datatype.clone());
            let body = self.get_body(element.syntax.get_function_declaration_expr().get_inside(), datatype.clone());
            function.statements = body.clone();
            // println!("function: {}", function.to_string());
            // println!("body: {}", body.len());
            self.functions.insert(name, (datatype, parameters, body));
            statements.push(function);
            self.pos += 1;
        }
        return statements;
    }
    fn get_datatype(&self, string: String) -> Datatype
    {
        let datatype = string.clone();
        let mut dim = Vec::<i32>::new();
        let mut is_array = false;
        while datatype.ends_with("]")
        {
            is_array = true;
            let datatype = util::remove_n_chars_from_behind(datatype.clone(), 1);
            let split = datatype.split("[");
            let split = split.last().unwrap();
            let number = split.parse::<i32>().unwrap();
            let datatype = util::remove_n_chars_from_behind(datatype.clone(), split.len());
            dim.push(number);
            util::remove_n_chars_from_behind(datatype.clone(), 1);
        }
        let datatype = util::get_datatype_from_string(datatype);
        let datatype = Datatype::new(datatype, dim, is_array);
        return datatype;
    }
    fn get_parameters(&self, function_declaration_expr: crate::parser::expression::FunctionDeclarationExpression) -> Arguments
    {
        let mut parameters = Arguments::new(vec![]);
        for parameter in function_declaration_expr.get_args()
        {
            let arg = parameter.syntax.get_arg_variable_expr();
            let name = arg.get_name();
            let datatype = self.get_datatype(arg.get_type());
            parameters.push(Statement::new(name, statement::StatementType::Argument, datatype.datatype, datatype.array_bounds, datatype.is_array));
        }
        return parameters;
    }
    fn get_body(&mut self, statements: Vec<Expression>, functiondatatype: Datatype) -> Vec<Statement>
    {
        let mut body = Vec::<Statement>::new();
        let mut returned = false;
        let mut variables = HashMap::<String, Datatype>::new();
        for statement in statements
        {
            if statement.is_call() // function call
            {
                let call = statement.syntax.get_call_expr();
                let name = call.get_name();
                if !self.functions.contains_key(&name)
                {
                    let err = util::get_line_of_position(self.context.clone(), statement.get_position());
                    panic!("function {} does not exist at {}:{}", name, err.0, err.1);
                }
                let mut arguments = Vec::<Statement>::new();
                let function = &self.functions.get(&name).unwrap().1;
                let args = call.get_args();
                let supressed_output = true;
                for i in 0..args.len()
                {
                    let arg = args.get(i).unwrap();
                    let farg = function.arguments.get(i).unwrap();
                    let same_datatype: bool;
                    if arg.is_literal()
                    {
                        same_datatype = self.is_datatype(farg.datatype.clone(), arg.clone(), supressed_output.clone(), &variables);
                    }
                    else
                    {
                        let argname = arg.syntax.get_variable_expr().get_name();
                        let arg = util::get_variable(argname.clone(), &variables);
                        same_datatype = util::same_datatype(farg.clone(), arg.clone());
                    }
                    if same_datatype
                    {
                        let datatype = self.get_datatype(farg.datatype.to_string());
                        let argument = Statement::new(String::from(""), StatementType::Argument, datatype.datatype, datatype.array_bounds, datatype.is_array);
                        arguments.push(argument);
                    }
                    else
                    {
                        if arg.is_literal()
                        {
                            let err = util::get_line_of_position(self.context.clone(), arg.get_position());
                            panic!("argument {} is not valid type at {}:{}", arg.clone().to_string(), err.0, err.1);
                        }
                        else
                        {
                            let argname = arg.syntax.get_variable_expr().get_name();
                            let err = util::get_line_of_position(self.context.clone(), arg.get_position());
                            panic!("argument {} is not valid type at {}:{}", argname, err.0, err.1);
                        }
                    }
                }
                let function = self.functions.get(&name).unwrap().0.clone();
                let call = Statement::new_call(name, arguments, function);
                body.push(call);
            }
            else
            if statement.is_variable_declaration()
            {
                let variable_declaration = statement.syntax.get_assignment_expr();
                let name = variable_declaration.get_name().syntax.get_variable_expr().get_name();
                let datatype = self.get_datatype(variable_declaration.get_type());
                let value = variable_declaration.get_value();
                let supress_output = true; // show error output if variable declaration is not valid (only visible if false)
                let test = self.is_datatype(datatype.clone(), value.clone(), supress_output, &variables);
                if !test
                {
                    let err = util::get_line_of_position(self.context.clone(),statement.get_position() + 2);
                    panic!("variable declaration {} {} is not valid at {}:{}", name, datatype.to_string(), err.0, err.1);
                }
                let mut variable = Statement::new(name.clone(), statement::StatementType::Variable, datatype.datatype, datatype.clone().array_bounds, datatype.clone().is_array);
                // println!("variable declaration {} {} is valid", name, datatype.to_string());
                variable.set_value(value.clone());
                body.push(variable);
                variables.insert(name.clone(), datatype.clone()); // TODO: possible to fix with body variable
            }
            else
            if statement.is_return()
            {
                // println!("return statement");
                let return_expr = statement.syntax.get_return_expr();
                let value = return_expr.get_value();
                let datatype = Datatype::new(StatementDatatype::Int, Vec::<i32>::new(), false); // TODO: get datatype of function
                let supress_output = true; // show error output if variable declaration is not valid (only visible if false)
                let test = self.is_datatype(datatype.clone(), value.clone(), supress_output, &variables);
                if !test
                {
                    let err = util::get_line_of_position(self.context.clone(),statement.get_position() + 2);
                    panic!("return {} {} is not valid at {}:{}", value.clone().to_string(), datatype.to_string(), err.0, err.1);
                }
                // let value = self.get_body(vec![value.clone()]);
                // let value = value.get(0).unwrap();
                if value.is_literal()
                {
                    if !value.is_integer_literal()
                    {
                        panic!("return value is not integer literal (not implemented)");
                    }
                    // println!("return value is integer literal");
                    // println!("{} | {}", value.to_string(), self.get_lit_datatype(value.to_string()).to_string());
                    if !util::get_lit_datatype(value.to_string()).is_same(&functiondatatype)
                    {
                        panic!("return value is not same as function datatype");
                    }
                    // println!("get_integer_literal (1)");
                    let value = value.syntax.get_integer_literal();
                    let datatype = self.get_datatype(datatype.to_string());
                    let value = Statement::new(value.to_string(), StatementType::Literal, datatype.datatype.clone(), datatype.array_bounds.clone(), datatype.is_array.clone());
                    let return_statement = Statement::new_return(value.clone(), datatype.clone());
                    body.push(return_statement);
                }
                else
                if value.is_variable()
                {
                    let value = value.syntax.get_variable_expr();
                    let name = value.get_name();
                    let datatype = util::get_variable(name.clone(), &variables);
                    if !datatype.is_same(&functiondatatype)
                    {
                        panic!("return value is not same as function datatype");
                    }
                    let value = Statement::new(name.clone(), StatementType::Variable, datatype.datatype.clone(), datatype.array_bounds.clone(), datatype.is_array.clone());
                    let return_statement = Statement::new_return(value.clone(), datatype.clone());
                    body.push(return_statement);
                }
                else
                {
                    panic!("return value is not literal or variable (not implemented)");
                }
                returned = true;
            }
            else
            if statement.is_variable_overwrite()
            {
                let variable_overwrite = statement.syntax.get_overwrite_variable_expr();
                let name = variable_overwrite.get_name();
                if !variables.contains_key(&name) // TODO: possible to fix with body variable
                {
                    let err = util::get_line_of_position(self.context.clone(),statement.get_position() + 2);
                    panic!("variable overwrite {} is not valid at {}:{}, because variable is not declared!", name, err.0, err.1);
                }
                // println!("variable overwrite {}", name);
                let datatype = util::get_variable(name.clone(), &variables);
                let value = variable_overwrite.get_value();
                // println!("value:|: {}", value.to_string());
                let supress_output = true; // show error output if variable declaration is not valid (only visible if false)
                let test = self.is_datatype(datatype.clone(), value.clone(), supress_output, &variables);
                if !test
                {
                    let err = util::get_line_of_position(self.context.clone(),statement.get_position() + 2);
                    panic!("variable overwrite {} {} is not valid at {}:{}", name, datatype.to_string(), err.0, err.1);
                }
                let mut variable = Statement::new(name.clone(), statement::StatementType::Variable, datatype.datatype, datatype.clone().array_bounds, datatype.clone().is_array);
                // println!("variable overwrite {} {} is valid", name, datatype.to_string());
                if value.is_literal()
                {
                    variable.set_value(value.clone());
                }
                else
                {
                    let value = self.parse_expr(value, &variables);
                    variable.statements.push(value.clone());
                }
                body.push(variable);
            }
        }
        if !returned && functiondatatype.datatype != StatementDatatype::Void
        {
            panic!("function does not return a value");
        }
        return body;
    }
    fn parse_expr(&self, expression: Expression, variables: &HashMap<String, Datatype>) -> Statement
    {
        if expression.is_call()
        {
            let call = expression.syntax.get_call_expr();
            let name = call.get_name();
            let _args = call.get_args(); // TODO: not yet inmplemented
            let return_type = self.get_function(name.clone(), 0);
            return Statement::new_call(name.clone(), vec![], return_type.0);
        }
        if expression.is_variable()
        {
            let variable = expression.syntax.get_variable_expr();
            let name = variable.get_name();
            let datatype = util::get_variable(name.clone(), &variables);
            return Statement::new(name.clone(), StatementType::Variable, datatype.datatype, datatype.array_bounds, datatype.is_array);
        }
        println!("expression: {}", expression.to_string());
        panic!("expression is not call");
    }
    fn test_variable_declaration(&self,datatype: Datatype, value: Expression, supress_output: bool, variables: &HashMap<String, Datatype>) -> bool
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
            return self.test_variable_declaration_binary(datatype, value, supress_output, &variables);
        }
        if value.is_call()
        {
            return self.test_variable_declaration_call(datatype, value, supress_output, &variables);
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
    fn test_variable_declaration_binary(&self, datatype: Datatype, value: Expression, supress_output: bool, variables: &HashMap<String, Datatype>) -> bool
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
            if self.is_datatype(boolstate.clone(), left.clone(), supress_output, &variables) && self.is_datatype(boolstate.clone(), right.clone(), supress_output, &variables)
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
            if self.is_datatype(stringstate.clone(), left.clone(), supress_output, &variables) && self.is_datatype(stringstate.clone(), right.clone(), supress_output, &variables)
            {
                return true;
            }
            if self.is_datatype(intstate.clone(), left.clone(), supress_output, &variables) && self.is_datatype(intstate.clone(), right.clone(), supress_output, &variables)
            {
                return true;
            }
            if self.is_datatype(charstate.clone(), left.clone(), supress_output, &variables) && self.is_datatype(charstate.clone(), right.clone(), supress_output, &variables)
            {
                return true;
            }
            if self.is_datatype(floatstate.clone(), left.clone(), supress_output, &variables) && self.is_datatype(floatstate.clone(), right.clone(), supress_output, &variables)
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
            if self.is_datatype(boolstate.clone(), left.clone(), supress_output, &variables) && self.is_datatype(boolstate.clone(), right.clone(), supress_output, &variables)
            {
                if !supress_output
                {
                    println!("cannot use binary operators except logical equals and not equals on bools");
                }
                return false;
            }
            if self.is_datatype(stringstate.clone(), left.clone(), supress_output, &variables) && self.is_datatype(stringstate.clone(), right.clone(), supress_output, &variables)
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
            if self.is_datatype(intstate.clone(), left.clone(), supress_output, &variables) && self.is_datatype(intstate.clone(), right.clone(), supress_output, &variables)
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
            if self.is_datatype(charstate.clone(), left.clone(), supress_output, &variables) && self.is_datatype(charstate.clone(), right.clone(), supress_output, &variables)
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
            if self.is_datatype(floatstate.clone(), left.clone(), supress_output, &variables) && self.is_datatype(floatstate.clone(), right.clone(), supress_output, &variables)
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
    fn is_datatype(&self, datatype: Datatype, value: Expression, supressed_output: bool, variables: &HashMap<String, Datatype>) -> bool
    {
        return self.test_variable_declaration(datatype, value, supressed_output, &variables);
    }
    fn test_variable_declaration_call(&self, datatype: Datatype, value: Expression, supress_output: bool, variables: &HashMap<String, Datatype>) -> bool {
        let call = value.clone().syntax.get_call_expr();
        let name = call.get_name();
        let args = call.get_args();
        let function = self.get_function(name, value.clone().get_position());
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
            if !self.test_variable_declaration(arg2, arg, false, &variables) {
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
    fn get_function(&self, name: String, pos: usize) -> (Datatype, Vec<Statement>) {
        let functions = self.functions.keys().map(|e| e.to_string() ).collect::<Vec<String>>();
        if !functions.contains(&name) {
            // println!("pos: {}", pos);
            let err = util::get_line_of_position(self.context.clone(), pos);
            panic!("function {} does not exist, called at {}:{}", name, err.0, err.1);
        }
        let data = self.functions.get(&name).unwrap().clone();
        let data = (data.0.clone(), data.1.arguments.clone());
        return data;
    }
}

