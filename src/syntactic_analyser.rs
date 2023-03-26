use std::collections::HashMap;
pub mod statement;
pub mod arguments;
mod standartfunctions;
mod typetest;
use statement::{
    Datatype,
    Statement,
    StatementType,
    StatementDatatype,
};

#[path ="tests/syntactic_analyser_tests.rs"]
mod unit_tests;

pub mod util;

use arguments::Arguments;
use super::{
    lexer,
    parser,
};
use lexer::token::LexerToken;
use parser::expression::Expression;

pub struct SyntaticAnalyser {
    pub statements: Vec<Expression>,
    pos: usize,
    functions: HashMap<String, (Datatype, Arguments, Vec::<Statement>)>,
    context: String,
    actual_datatype: Datatype,
    std_function_count: HashMap<String, usize>,
}
impl SyntaticAnalyser {
    pub fn new(statements: Vec<Expression>, context: String, std_included: bool) -> SyntaticAnalyser {
        // println!("start syntactic analyser");
        let mut functions = HashMap::<String, (Datatype, Arguments, Vec::<Statement>)>::new();
        let mut std_function_count = HashMap::<String, usize>::new();
        if std_included
        {
            standartfunctions::generate_functions(&mut functions);
            for key in functions.keys()
            {
                std_function_count.insert(key.clone(), 0);
            }
        }
        SyntaticAnalyser {
            statements: statements,
            pos: 0,
            functions,
            context: context,
            actual_datatype: Datatype::new(StatementDatatype::Void, vec![], false),
            std_function_count,
        }
    }
    pub fn get_std_functions(&self) -> HashMap<String, usize>
    {
        self.std_function_count.clone()
    }
    pub fn analyse(&mut self) -> (Vec<Statement>, HashMap<String, (Datatype, Arguments, Vec::<Statement>)>)
    {
        println!("functions: {:?}", self.functions.keys());
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
            if self.functions.contains_key(&name)
            {
                panic!("function already exists: {}", name);
            }
            let datatype = util::get_datatype(function_declaration_expr.get_type(), true);
            let parameters = util::get_parameters(function_declaration_expr);
            let mut function = Statement::new_datatype(name.clone(), StatementType::Function, datatype.clone());
            self.actual_datatype = datatype.clone();
            let body = self.get_body(element.syntax.get_function_declaration_expr().get_inside(),
                HashMap::<String, Datatype>::new(), datatype.clone(), parameters.clone(), true);
            function.statements = body.clone();
            self.functions.insert(name, (datatype, parameters, body));
            // println!("function: {}", function.to_string());
            // println!("body: {}", body.len());
            statements.push(function);
            self.pos += 1;
        }
        return (statements, self.functions.clone());
    }
    pub fn get_body(&mut self, statements: Vec<Expression>, variables: HashMap<String,Datatype>, functiondatatype: Datatype, args: Arguments, _of_function: bool) -> Vec<Statement>
    {
        let mut body = Vec::<Statement>::new();
        let mut returned = false;
        let mut variables = variables;
        let _this_args = args.clone();
        for arg in args.clone().arguments
        {
            variables.insert(arg.name, arg.datatype);
        }
        for statement in statements
        {
            if returned
            {
                panic!("code after return statement");
            }
            if statement.is_call() // function call
            {
                let call = statement.syntax.get_call_expr();
                let name = call.get_name();
                let mut is_std = false;
                if standartfunctions::is_std_function(&name)
                {
                    is_std = true;
                }
                if !self.functions.contains_key(&name) && !is_std
                {
                    let err = util::get_line_of_position(self.context.clone(), statement.get_position());
                    panic!("function {} does not exist at {}:{}", name, err.0, err.1);
                }
                // println!("call: {}", name);
                // println!("functions: {:?}", self.functions.keys());
                if is_std
                {
                    for name in standartfunctions::get_variants(&name)
                    {
                        println!("name: {}", name);
                        if self.functions.contains_key(&name)
                        {
                            let function = self.functions.get(&name).unwrap();
                            let fargs = function.1.clone();
                            let args = call.get_args();
                            if fargs.arguments.len() != args.len()
                            {
                                continue;
                            }
                            let mut is_possible = true;
                            for i in 0..args.len()
                            {
                                let farg = fargs.arguments.get(i).unwrap();
                                let arg = args.get(i).unwrap();
                                if !self.is_possible(&arg, farg, true, &variables)
                                {
                                    is_possible = false;
                                    break;
                                }
                            }
                            if is_possible
                            {
                                println!("here!");
                                let mut call = call.clone();
                                call.set_name(name.clone());
                                self.generate_call(name, call, &statement, &variables, &mut body);
                                break;
                            }
                        }
                    }
                }
                else
                {
                    self.generate_call(name, call, &statement, &variables, &mut body);
                }
            }
            else
            if statement.is_return()
            {
                // println!("return statement");
                let return_expr = statement.syntax.get_return_expr();
                let value = return_expr.get_value();
                let datatype = functiondatatype.clone();
                // let value = self.get_body(vec![value.clone()]);
                // let value = value.get(0).unwrap();
                let value = util::generate_binary(value, &variables, &self.functions);
                // println!("{}", value.datatype);
                if !datatype.is_same(&value.datatype)
                {
                    let err = util::get_line_of_position(self.context.clone(),statement.get_position() + 2);
                    panic!("return {} {} is not valid at {}:{}", value.clone().to_string(), datatype.to_string(), err.0, err.1);
                }
                let return_statement = Statement::new_return(value, datatype.clone());
                body.push(return_statement);
                returned = true;
            }
            else
            if statement.is_variable_overwrite() || statement.is_variable_declaration()
            {
                self.analyse_variable(&statement, &mut variables, &mut body);
            }
            else
            if statement.is_if()
            {
                let if_statement = statement.syntax.get_if_expr();
                let condition = if_statement.get_condition();
                let condition = util::generate_binary(condition.clone(), &variables, &self.functions);
                if condition.datatype.datatype != StatementDatatype::Bool
                {
                    let err = util::get_line_of_position(self.context.clone(),statement.get_position() + 2);
                    panic!("if condition {} is not valid at {}:{}", condition.clone().to_string(), err.0, err.1);
                }
                let if_then = if_statement.get_then_branch();
                let if_body = self.get_body(if_then.clone(), variables.clone(), functiondatatype.clone(), args.clone(), false);
                let mut else_body = Vec::<Statement>::new();
                if if_statement.has_else_branch()
                {
                    let if_else = if_statement.get_else_branch();
                    else_body = self.get_body(if_else.clone(), variables.clone(), functiondatatype.clone(), args.clone(), false);
                }
                let if_statement = Statement::new_if(condition, if_body, else_body);
                body.push(if_statement);
            }
            else if statement.is_array_overwrite()
            {
                let var_expr = statement.syntax.get_overwrite_array_expr();
                let name = var_expr.get_name();
                let value = var_expr.get_value();
                let value = util::generate_binary(value.clone(), &variables, &self.functions);
                let indices = var_expr.get_indices();
                let mut indices_statement = Vec::<Statement>::new();
                for index in indices
                {
                    let bin = util::generate_binary(index.clone(), &variables, &self.functions);
                    indices_statement.push(bin);
                }
                let datatype = util::get_variable(name.clone(), &variables);
                if !datatype.is_array
                {
                    let err = util::get_line_of_position(self.context.clone(),statement.get_position() + 2);
                    panic!("array overwrite {} is not valid at {}:{}", name, err.0, err.1);
                }
                let mut statement = Statement::new(name.clone(), statement::StatementType::ArrayOverwrite, datatype.datatype, datatype.clone().array_bounds, datatype.clone().is_array);
                statement.statements.push(value.clone());
                statement.statements.append(&mut indices_statement);
                body.push(statement);
            }
            else if statement.is_for_expr()
            {
                let for_expr = statement.syntax.get_for_expr();
                let init_expr = for_expr.get_init_expression();
                let test_expr = for_expr.get_test_expression();
                let update_expr = for_expr.get_update_expression();
                let init_expr = init_expr.get(0);
                if init_expr.is_none()
                {
                    panic!("Init Expression is None");
                }
                let init_expr = init_expr.unwrap();
                let init_expr = self.analyse_variable(init_expr, &mut variables, &mut body);
                // TODO: use the expressions
            }
        }
        return body;
    }
    fn analyse_variable(&mut self, statement: &Expression, variables: &mut HashMap<String,Datatype>, body: &mut Vec<Statement>)
    {
        if statement.is_variable_overwrite()
        {
            self.analyse_overwrite(statement,variables, body);
        }
        else if statement.is_variable_declaration()
        {
            self.analyse_declaration(statement, variables, body);
        }
        else
        {
            panic!("Expression is not regarding variables");
        }
    }
    fn analyse_declaration(&mut self, statement: &Expression, variables: &mut HashMap<String,Datatype>, body: &mut Vec<Statement>)
    {
        let variable_declaration = statement.syntax.get_assignment_expr();
        let name = variable_declaration.get_name().syntax.get_variable_expr().get_name();
        if variables.contains_key(&name)
        {
            let err = util::get_line_of_position(self.context.clone(),statement.get_position() + 2);
            panic!("variable {} already exists at {}:{}", name, err.0, err.1);
        }
        let datatype = util::get_datatype(variable_declaration.get_type(), false);
        let value = variable_declaration.get_value();
        let val = util::generate_binary(value.clone(), &variables, &self.functions);
        println!("datatype: {} ,  {}", val.datatype.to_string(), datatype.is_array);
        if !datatype.is_same(&val.datatype) && !(value.is_integer_literal() && value.syntax.get_integer_literal() == 0)
        {
            let err = util::get_line_of_position(self.context.clone(),statement.get_position() + 2);
            println!("{}{}", datatype.clone().to_string(), "");
            panic!("variable declaration {} {} is not valid at {}:{}", name, datatype.to_string(), err.0, err.1);
        }
        let mut variable = Statement::new(name.clone(), statement::StatementType::Variable, datatype.datatype, datatype.clone().array_bounds, datatype.clone().is_array);
        // println!("variable declaration {} {} is valid", name, datatype.to_string());
        variable.set_value(value.clone(), &variables, &self.functions);
        body.push(variable.clone());
        // println!("variable: {}", variable.to_string());
        variables.insert(name.clone(), datatype.clone()); // TODO: possible to fix with body variable
    }
    fn analyse_overwrite(&mut self, statement: &Expression, variables: &mut HashMap<String,Datatype>, body: &mut Vec<Statement>)
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
        let value = util::generate_binary(value.clone(), &variables, &self.functions);
        if !datatype.is_same(&value.datatype)
        {
            let err = util::get_line_of_position(self.context.clone(),statement.get_position() + 2);
            panic!("variable overwrite {} {} is not valid at {}:{}", name, datatype.to_string(), err.0, err.1);
        }
        let mut variable = Statement::new(name.clone(), statement::StatementType::Variable, datatype.datatype, datatype.clone().array_bounds, datatype.clone().is_array);
        // println!("variable overwrite {} {} is valid", name, datatype.to_string());

        variable.statements.push(value.clone());
        body.push(variable.clone());
        println!("variable: {}", variable.to_string());
    }
    fn generate_call(&mut self, name: String, call: parser::expression::CallExpression, statement: &Expression, variables: &HashMap<String, Datatype>, body: &mut Vec<Statement>) {
        let mut arguments = Vec::<Statement>::new();
        let function = &self.functions.get(&name).unwrap().1;
        let args = call.get_args();
        let supressed_output = true;
        if args.len() != function.arguments.len()
        {
            let err = util::get_line_of_position(self.context.clone(), statement.get_position());
            panic!("function {} expects {} arguments, but {} were given at {}:{}", name, function.arguments.len(), args.len(), err.0, err.1);
        }
        for i in 0..args.len()
        {
            let arg = args.get(i).unwrap();
            let farg = function.arguments.get(i).unwrap();
            let same_datatype: bool = self.is_possible(arg, farg, supressed_output, variables);
            if same_datatype
            {
                // let datatype = util::get_datatype(farg.datatype.to_string(), false);
                //let argument = Statement::new(String::from(""), StatementType::Argument, datatype.datatype, datatype.array_bounds, datatype.is_array);
                let arg = util::generate_binary(arg.clone(), variables, &self.functions);
                arguments.push(arg);
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
        if standartfunctions::is_std_function(&name)
        {
            self.increment_std_function_count(&name);
        }
        let function = self.functions.get(&name).unwrap().0.clone();
        let call = Statement::new_call(name, arguments, function);
        body.push(call.clone());
        println!("call {}", call.to_string());
    }

    fn is_possible(&self, arg: &Expression, farg: &Statement, supressed_output: bool, variables: &HashMap<String, Datatype>) -> bool{
        if arg.is_literal()
        {
            return typetest::is_datatype(farg.datatype.clone(), arg.clone(), supressed_output.clone(), variables, &self.functions);
        }
        else
        {
            let argname = arg.syntax.get_variable_expr().get_name();
            let arg = util::get_variable(argname.clone(), variables);
            return util::same_datatype(farg.clone(), arg.clone());
        }
    }
    fn increment_std_function_count(&mut self, name: &str)
    {
        if self.std_function_count.contains_key(name)
        {
            let count = self.std_function_count.get(name).unwrap();
            self.std_function_count.insert(name.to_string(), count + 1);
        }
        else
        {
            self.std_function_count.insert(name.to_string(), 1);
        }
    }
}

