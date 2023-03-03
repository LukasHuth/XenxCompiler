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
}
impl SyntaticAnalyser {
    pub fn new(statements: Vec<Expression>) -> SyntaticAnalyser {
        println!("start syntactic analyser");
        SyntaticAnalyser {
            statements: statements,
            pos: 0,
            variables: HashMap::<String, Datatype>::new(),
            functions: HashMap::<String, (Datatype, Arguments, Vec::<Statement>)>::new(),
        }
    }
    pub fn analyse(&mut self) -> Vec<Statement>
    {
        // let mut statements = Vec::<Statement>::new();
        println!("statements: {}", self.statements.len());
        let mut statements = Vec::<Statement>::new();
        let test = self.get_datatype(String::from("int"));
        // println!("test: {}", test.to_string());
        while self.pos < self.statements.len()
        {
            let element = self.statements.get(self.pos).unwrap();
            // println!("{}", element.clone().to_string());
            if !element.is_function_declaration()
            {
                println!("first element is not a function declaration");
                return vec![];
            }
            let function_declaration_expr = element.syntax.get_function_declaration_expr();
            let name = function_declaration_expr.get_name();
            let datatype = self.get_datatype(function_declaration_expr.get_type());
            // paused here, next thing i wanted to do is to move parameter extraction to a function
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
        // println!("datatype: {} ||", datatype);
        let datatype = self.get_datatype_from_string(datatype);
        // println!("dim: {}", datatype.to_string());
        let datatype = Datatype::new(datatype, dim, is_array);
        return datatype;
    }
    fn get_datatype_from_string(&self, datastring: String) -> StatementDatatype
    {
        let mut datatype = StatementDatatype::Void;
        let datastring = datastring.clone();
        let datastring = datastring.to_lowercase();
        let datastring = datastring.trim();
        // println!("datastring: {}", datastring);
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
        // println!("datatype: {} <-", datatype.to_string());
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
        // println!("get parameters");
        let mut parameters = Arguments::new(vec![]);
        for parameter in function_declaration_expr.get_args()
        {
            // println!("before");
            let arg = parameter.syntax.get_arg_variable_expr();
            // println!("after");
            let name = arg.get_name();
            let datatype = self.get_datatype(arg.get_type());
            parameters.push(Statement::new(name, statement::StatementType::Argument, datatype.datatype, datatype.array_bounds, datatype.is_array));
        }
        return parameters;
    }
    fn get_lit_datatype(&self, string: String) -> StatementDatatype
    {
        let mut datatype = StatementDatatype::Void;
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
                    panic!("function {} does not exist", name);
                }
                let mut arguments = Vec::<Statement>::new();
                let function = &self.functions.get(&name).unwrap().1;
                let args = call.get_args();
                let supressed_output = true;
                for i in 0..args.len()
                {
                    let arg = args.get(i).unwrap();
                    let farg = function.arguments.get(i).unwrap();
                    // println!(":{} {}", farg.datatype.clone(), arg.clone().to_string());
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
                    // let same_datatype = self.is_datatype(farg.datatype.clone(), arg.clone(), supressed_output.clone());
                    // if self.test_variable_declaration(farg.datatype.clone(), arg.clone(), supress_output)
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
                            panic!("argument {} is not valid type", arg.clone().to_string());
                        }
                        else
                        {
                            let argname = arg.syntax.get_variable_expr().get_name();
                            panic!("argument {} is not valid type", argname);
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
                // println!("variable declaration {} {}", name, datatype.to_string());
                let value = variable_declaration.get_value();
                let valuedatatype = self.get_datatype(value.to_string());
                // println!("value datatype: {}", valuedatatype.to_string());
                let supress_output = true; // show error output if variable declaration is not valid (only visible if false)
                let test = self.is_datatype(datatype.clone(), value, supress_output);
                if !test
                {
                    panic!("variable declaration {} {} is not valid", name, datatype.to_string());
                }
                let variable = Statement::new(name.clone(), statement::StatementType::Variable, datatype.datatype, datatype.clone().array_bounds, datatype.clone().is_array);
                body.push(variable);
                self.variables.insert(name.clone(), datatype.clone());
            }
        }
        return vec![];
        // todo!()
    }
    fn test_variable_declaration(&self,datatype: Datatype, value: Expression, supressOutput: bool) -> bool
    {
        if value.is_literal()
        {
            return self.test_variable_declaration_literal(datatype, value, supressOutput);
        }
        else
        if value.is_variable()
        {
            return self.test_variable_declaration_variable(datatype, value, supressOutput);
        }
        else
        if value.is_binary()
        {
            return self.test_variable_declaration_binary(datatype, value, supressOutput);
        }
        else
        {
            if !supressOutput
            {
                println!("variable declaration is not a literal or variable");
            }
            return false;
        }
    }
    fn test_variable_declaration_binary(&self, datatype: Datatype, value: Expression, supressOutput: bool) -> bool
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
            if self.is_datatype(boolstate.clone(), left.clone(), supressOutput) && self.is_datatype(boolstate.clone(), right.clone(), supressOutput)
            {
                return true;
            }
            if operator.token == LexerToken::AmpersandAmpersand || operator.token == LexerToken::PipePipe
            {
                if !supressOutput
                {
                    println!("cannot use && or || on other that bool");
                }
                return false;
            }
            if self.is_datatype(stringstate.clone(), left.clone(), supressOutput) && self.is_datatype(stringstate.clone(), right.clone(), supressOutput)
            {
                return true;
            }
            if self.is_datatype(intstate.clone(), left.clone(), supressOutput) && self.is_datatype(intstate.clone(), right.clone(), supressOutput)
            {
                return true;
            }
            if self.is_datatype(charstate.clone(), left.clone(), supressOutput) && self.is_datatype(charstate.clone(), right.clone(), supressOutput)
            {
                return true;
            }
            if self.is_datatype(floatstate.clone(), left.clone(), supressOutput) && self.is_datatype(floatstate.clone(), right.clone(), supressOutput)
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
            if self.is_datatype(boolstate.clone(), left.clone(), supressOutput) && self.is_datatype(boolstate.clone(), right.clone(), supressOutput)
            {
                if !supressOutput
                {
                    println!("cannot use binary operators except logical equals and not equals on bools");
                }
                return false;
            }
            if self.is_datatype(stringstate.clone(), left.clone(), supressOutput) && self.is_datatype(stringstate.clone(), right.clone(), supressOutput)
            {
                if datatype.datatype == StatementDatatype::String
                {
                    return true;
                }
                else
                {
                    if !supressOutput
                    {
                        println!("cannot use binary operators on strings except logical equals and not equals");
                    }
                    return false;
                }
            }
            if self.is_datatype(intstate.clone(), left.clone(), supressOutput) && self.is_datatype(intstate.clone(), right.clone(), supressOutput)
            {
                if datatype.datatype == StatementDatatype::Int
                {
                    return true;
                }
                else
                {
                    if !supressOutput
                    {
                        println!("cannot use binary operators on ints except logical equals and not equals");
                    }
                    return false;
                }
            }
            if self.is_datatype(charstate.clone(), left.clone(), supressOutput) && self.is_datatype(charstate.clone(), right.clone(), supressOutput)
            {
                if datatype.datatype == StatementDatatype::Char
                {
                    return true;
                }
                else
                {
                    if !supressOutput
                    {
                        println!("cannot use binary operators on chars except logical equals and not equals");
                    }
                    return false;
                }
            }
            if self.is_datatype(floatstate.clone(), left.clone(), supressOutput) && self.is_datatype(floatstate.clone(), right.clone(), supressOutput)
            {
                if datatype.datatype == StatementDatatype::Float
                {
                    return true;
                }
                else
                {
                    if !supressOutput
                    {
                        println!("cannot use binary operators on floats except logical equals and not equals");
                    }
                    return false;
                }
            }
        }
        return false;
    }
    fn is_datatype(&self, datatype: Datatype, value: Expression, supressedOutput: bool) -> bool
    {
        return self.test_variable_declaration(datatype, value, supressedOutput);
    }
    fn test_variable_declaration_variable(&self, datatype: Datatype, value: Expression, supressOutput: bool) -> bool
    {
        let variable = value.syntax.get_variable_expr();
        let name = variable.get_name(); 
        let variable = self.get_variable(name);
        // println!("var: {}", variable);
        if datatype.datatype != variable.datatype && datatype.is_array != variable.is_array && datatype.array_bounds != variable.array_bounds
        {
            if !supressOutput
            {
                println!("variable declaration is not the same type as the variable");
            }
            return false;
        }
        return true;
    }
    fn get_variable(&self, name: String) -> Datatype
    {
        self.variables.get(&name).unwrap().clone()
    }
    fn test_variable_declaration_literal(&self, datatype: Datatype, value: Expression, supressOutput: bool) -> bool
    {
        match datatype.datatype
        {
            StatementDatatype::Int => {
                if !value.is_integer_literal()
                {
                    if !supressOutput
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
                    if !supressOutput
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
                    if !supressOutput
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
                    if !supressOutput
                    {
                        println!("variable is not a bool");
                    }
                    return false;
                }
                return true;
            }
            StatementDatatype::Void => {
                if !supressOutput
                {
                    println!("variable is void");
                }
                return false;
            }
            StatementDatatype::Char => {
                if !value.is_char_literal()
                {
                    if !supressOutput
                    {
                        println!("variable is not a char");
                    }
                    return false;
                }
                return true;
            }
            _ => {
                if !supressOutput
                {
                    println!("variable is not a valid datatype");
                }
                return false;
            }
        }
    }

    fn same_datatype(&self, arg1: &Statement, arg2: Datatype) -> bool {
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
}

