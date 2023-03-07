use std::collections::HashMap;
pub mod statement;
pub mod arguments;
use statement::Statement;
use statement::StatementType;
use statement::StatementDatatype;

use self::arguments::Arguments;
use self::statement::Datatype;

use super::lexer::token::LexerToken;
use super::parser::expression::Expression;

pub struct SyntaticAnalyser {
    pub statements: Vec<Expression>,
    pos: usize,
    variables: HashMap<String, Datatype>,
    functions: HashMap<String, (Datatype, Arguments, Vec::<Statement>)>,
    context: String,
}
impl SyntaticAnalyser {
    pub fn new(statements: Vec<Expression>, context: String) -> SyntaticAnalyser {
        println!("start syntactic analyser");
        SyntaticAnalyser {
            statements: statements,
            pos: 0,
            variables: HashMap::<String, Datatype>::new(),
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
            let function = Statement::new_datatype(name.clone(), StatementType::Function, datatype.clone());
            let body = self.get_body(element.syntax.get_function_declaration_expr().get_inside());
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
            let datatype = self.remove_n_chars_from_behind(datatype.clone(), 1);
            let split = datatype.split("[");
            let split = split.last().unwrap();
            let number = split.parse::<i32>().unwrap();
            let datatype = self.remove_n_chars_from_behind(datatype.clone(), split.len());
            dim.push(number);
            self.remove_n_chars_from_behind(datatype.clone(), 1);
        }
        let datatype = self.get_datatype_from_string(datatype);
        let datatype = Datatype::new(datatype, dim, is_array);
        return datatype;
    }
    fn get_datatype_from_string(&self, datastring: String) -> StatementDatatype
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
    fn remove_n_chars_from_behind(&self, string: String, n: usize) -> String
    {
        let mut string = string;
        for _ in 0..n
        {
            string.pop();
        }
        return string;
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
    #[allow(dead_code)]
    fn get_lit_datatype(&self, string: String) -> StatementDatatype
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
        return datatype;
    }
    fn get_body(&mut self, statements: Vec<Expression>) -> Vec<Statement>
    {
        let mut body = Vec::<Statement>::new();
        for statement in statements
        {
            if statement.is_call() // function call
            {
                let call = statement.syntax.get_call_expr();
                let name = call.get_name();
                if !self.functions.contains_key(&name)
                {
                    let err = self.get_line_of_position(statement.get_position());
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
                        same_datatype = self.is_datatype(farg.datatype.clone(), arg.clone(), supressed_output.clone());
                    }
                    else
                    {
                        let argname = arg.syntax.get_variable_expr().get_name();
                        let arg = self.get_variable(argname.clone());
                        same_datatype = self.same_datatype(farg.clone(), arg.clone());
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
                            let err = self.get_line_of_position(arg.get_position());
                            panic!("argument {} is not valid type at {}:{}", arg.clone().to_string(), err.0, err.1);
                        }
                        else
                        {
                            let argname = arg.syntax.get_variable_expr().get_name();
                            let err = self.get_line_of_position(arg.get_position());
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
                let test = self.is_datatype(datatype.clone(), value, supress_output);
                if !test
                {
                    let err = self.get_line_of_position(statement.get_position() + 2);
                    panic!("variable declaration {} {} is not valid at {}:{}", name, datatype.to_string(), err.0, err.1);
                }
                let variable = Statement::new(name.clone(), statement::StatementType::Variable, datatype.datatype, datatype.clone().array_bounds, datatype.clone().is_array);
                body.push(variable);
                self.variables.insert(name.clone(), datatype.clone());
            }
        }
        return vec![];
    }
    fn test_variable_declaration(&self,datatype: Datatype, value: Expression, supress_output: bool) -> bool
    {
        if value.is_literal()
        {
            return self.test_variable_declaration_literal(datatype, value, supress_output);
        }
        else
        if value.is_variable()
        {
            return self.test_variable_declaration_variable(datatype, value, supress_output);
        }
        else
        if value.is_binary()
        {
            return self.test_variable_declaration_binary(datatype, value, supress_output);
        }
        if value.is_call()
        {
            return self.test_variable_declaration_call(datatype, value, supress_output);
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
    fn test_variable_declaration_binary(&self, datatype: Datatype, value: Expression, supress_output: bool) -> bool
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
            if self.is_datatype(boolstate.clone(), left.clone(), supress_output) && self.is_datatype(boolstate.clone(), right.clone(), supress_output)
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
            if self.is_datatype(stringstate.clone(), left.clone(), supress_output) && self.is_datatype(stringstate.clone(), right.clone(), supress_output)
            {
                return true;
            }
            if self.is_datatype(intstate.clone(), left.clone(), supress_output) && self.is_datatype(intstate.clone(), right.clone(), supress_output)
            {
                return true;
            }
            if self.is_datatype(charstate.clone(), left.clone(), supress_output) && self.is_datatype(charstate.clone(), right.clone(), supress_output)
            {
                return true;
            }
            if self.is_datatype(floatstate.clone(), left.clone(), supress_output) && self.is_datatype(floatstate.clone(), right.clone(), supress_output)
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
            if self.is_datatype(boolstate.clone(), left.clone(), supress_output) && self.is_datatype(boolstate.clone(), right.clone(), supress_output)
            {
                if !supress_output
                {
                    println!("cannot use binary operators except logical equals and not equals on bools");
                }
                return false;
            }
            if self.is_datatype(stringstate.clone(), left.clone(), supress_output) && self.is_datatype(stringstate.clone(), right.clone(), supress_output)
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
            if self.is_datatype(intstate.clone(), left.clone(), supress_output) && self.is_datatype(intstate.clone(), right.clone(), supress_output)
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
            if self.is_datatype(charstate.clone(), left.clone(), supress_output) && self.is_datatype(charstate.clone(), right.clone(), supress_output)
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
            if self.is_datatype(floatstate.clone(), left.clone(), supress_output) && self.is_datatype(floatstate.clone(), right.clone(), supress_output)
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
    fn is_datatype(&self, datatype: Datatype, value: Expression, supressed_output: bool) -> bool
    {
        return self.test_variable_declaration(datatype, value, supressed_output);
    }
    fn test_variable_declaration_variable(&self, datatype: Datatype, value: Expression, supress_output: bool) -> bool
    {
        println!("test for: {}", value.to_string());
        let variable = value.syntax.get_variable_expr();
        let name = variable.get_name(); 
        // println!("test for: {}", name);
        let variable = self.get_variable(name);
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
    fn get_variable(&self, name: String) -> Datatype
    {
        self.variables.get(&name).unwrap().clone()
    }
    #[allow(unreachable_patterns)]
    fn test_variable_declaration_literal(&self, datatype: Datatype, value: Expression, supress_output: bool) -> bool
    {
        println!("test for: {}", value.to_string());
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

    fn same_datatype(&self, arg1: Statement, arg2: Datatype) -> bool {
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

    fn test_variable_declaration_call(&self, datatype: Datatype, value: Expression, supress_output: bool) -> bool {
        let call = value.clone().syntax.get_call_expr();
        let name = call.get_name();
        let args = call.get_args();
        let function = self.get_function(name, value.clone().get_position());
        println!("function: {}", function.0.to_string());
        println!("datatype: {}", datatype.to_string());
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
            if !self.test_variable_declaration(arg2, arg, false) {
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
            println!("pos: {}", pos);
            let err = self.get_line_of_position(pos);
            panic!("function {} does not exist, called at {}:{}", name, err.0, err.1);
        }
        let data = self.functions.get(&name).unwrap().clone();
        let data = (data.0.clone(), data.1.arguments.clone());
        return data;
    }
    fn get_line_of_position(&self, position: usize) -> (usize, usize) {
        let mut start = 0;
        let mut line_ = 1;
        let mut pos = 0;
        while pos < self.context.chars().count()
        {
            if pos == position
            {
                return (line_, start-1);
            }
            let line = self.context.chars().nth(pos).unwrap();
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
}

