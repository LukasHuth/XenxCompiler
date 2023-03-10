use std::collections::HashMap;
pub mod statement;
pub mod arguments;
mod typetest;
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
    pub fn analyse(&mut self) -> (Vec<Statement>, HashMap<String, (Datatype, Arguments, Vec::<Statement>)>)
    {
        // println!("statements: {}", self.statements.len());
        let mut statements = Vec::<Statement>::new();
        while self.pos < self.statements.len()
        {
            let element = self.statements.get(self.pos).unwrap();
            if !element.is_function_declaration()
            {
                panic!("expected function declaration");
            }
            let function_declaration_expr = element.syntax.get_function_declaration_expr();
            let name = function_declaration_expr.get_name();
            let datatype = self.get_datatype(function_declaration_expr.get_type());
            let parameters = self.get_parameters(function_declaration_expr);
            let mut function = Statement::new_datatype(name.clone(), StatementType::Function, datatype.clone());
            let body = self.get_body(element.syntax.get_function_declaration_expr().get_inside(), datatype.clone(), parameters.clone());
            function.statements = body.clone();
            // println!("function: {}", function.to_string());
            // println!("body: {}", body.len());
            self.functions.insert(name, (datatype, parameters, body));
            statements.push(function);
            self.pos += 1;
        }
        return (statements, self.functions.clone());
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
    fn get_body(&mut self, statements: Vec<Expression>, functiondatatype: Datatype, args: Arguments) -> Vec<Statement>
    {
        let mut body = Vec::<Statement>::new();
        let mut returned = false;
        let mut variables = HashMap::<String, Datatype>::new();
        for arg in args.arguments
        {
            variables.insert(arg.name, arg.datatype);
        }
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
                        same_datatype = typetest::is_datatype(farg.datatype.clone(), arg.clone(), supressed_output.clone(), &variables, &self.functions);
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
                let test = typetest::is_datatype(datatype.clone(), value.clone(), supress_output, &variables, &self.functions);
                if !test
                {
                    let err = util::get_line_of_position(self.context.clone(),statement.get_position() + 2);
                    println!("{}{}", datatype.clone().to_string(), "");
                    panic!("variable declaration {} {} is not valid at {}:{}", name, datatype.to_string(), err.0, err.1);
                }
                let mut variable = Statement::new(name.clone(), statement::StatementType::Variable, datatype.datatype, datatype.clone().array_bounds, datatype.clone().is_array);
                // println!("variable declaration {} {} is valid", name, datatype.to_string());
                variable.set_value(value.clone(), &variables, &self.functions);
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
                let test = typetest::is_datatype(datatype.clone(), value.clone(), supress_output, &variables, &self.functions);
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
                let test = typetest::is_datatype(datatype.clone(), value.clone(), supress_output, &variables, &self.functions);
                if !test
                {
                    let err = util::get_line_of_position(self.context.clone(),statement.get_position() + 2);
                    panic!("variable overwrite {} {} is not valid at {}:{}", name, datatype.to_string(), err.0, err.1);
                }
                let mut variable = Statement::new(name.clone(), statement::StatementType::Variable, datatype.datatype, datatype.clone().array_bounds, datatype.clone().is_array);
                // println!("variable overwrite {} {} is valid", name, datatype.to_string());
                if value.is_literal()
                {
                    variable.set_value(value.clone(), &variables, &self.functions);
                }
                else
                if value.is_call()
                {
                    variable.set_value(value.clone(), &variables, &self.functions);
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
        if expression.is_binary()
        {
            let binary = util::generate_binary(expression.clone(), &variables, &self.functions);
            return binary;
        }
        println!("expression: {}", expression.to_string());
        panic!("expression is not call");
    }
    
}

