use std::collections::HashMap;
pub mod statement;
use statement::Statement;
use statement::StatementDatatype;

use self::statement::Datatype;

use super::parser::expression::Expression;

pub struct SyntaticAnalyser {
    pub statements: Vec<Expression>,
    pos: usize,
    variables: HashMap<String, Datatype>,

}
impl SyntaticAnalyser {
    pub fn new(statements: Vec<Expression>) -> SyntaticAnalyser {
        println!("start syntactic analyser");
        SyntaticAnalyser {
            statements: statements,
            pos: 0,
            variables: HashMap::<String, Datatype>::new(),
        }
    }
    pub fn analyse(&mut self) -> Vec<Statement>
    {
        // let mut statements = Vec::<Statement>::new();
        println!("statements: {}", self.statements.len());
        let first_element = self.statements.get(0).unwrap();
        if !first_element.is_function_declaration()
        {
            println!("first element is not a function declaration");
            return vec![];
        }
        let function_declaration_expr = first_element.syntax.get_function_declaration_expr();
        let name = function_declaration_expr.get_name();
        let datatype = self.get_datatype(function_declaration_expr.get_type());
        // paused here, next thing i wanted to do is to move parameter extraction to a function
        let mut parameters = Vec::<Statement>::new();
        for parameter in function_declaration_expr.get_args()
        {
            let arg = parameter.syntax.get_arg_variable_expr();
            let name = arg.get_name();
            let mut datatype = arg.get_type();
        }
        // let syntax = expression.syntax.clone();
        // println!("expression: {:#?}", expression);
        // println!("expression: {}", expression.to_string());
        return vec![];
    }
    fn get_datatype(&self, string: String) -> Datatype
    {
        let mut datatype = string.clone();
        let mut dim = Vec::<i32>::new();
        let mut is_array = false;
        while datatype.ends_with("]")
        {
            is_array = true;
            let datatype = self.remove_n_chars_from_behind(datatype.clone(), 1);
            let mut split = datatype.split("[");
            let split = split.last().unwrap();
            let number = split.parse::<i32>().unwrap();
            let datatype = self.remove_n_chars_from_behind(datatype.clone(), split.len());
            dim.push(number);
            let datatype = self.remove_n_chars_from_behind(datatype.clone(), 1);
        }
        let datatype = self.get_datatype_from_string(datatype);
        let datatype = Datatype::new(datatype, dim, is_array);
        return datatype;
    }
    fn get_datatype_from_string(&self, datastring: String) -> StatementDatatype
    {
        let mut datatype = StatementDatatype::Void;
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
}