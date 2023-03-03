use std::collections::HashMap;
pub mod lexer;
pub mod parser;
pub mod syntactic_analyser;
mod test;
// https://norasandler.com/2017/11/29/Write-a-Compiler.html
#[allow(dead_code)]
fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let dash_args = export_arguments(args);
    let args = dash_args.clone().1;
    let dash_args = dash_args.clone().0;
    println!("dash args: {:?}", dash_args);
    if args.len() < 2
    {
        println!("Usage: {} <input>", args[0]);
    }
    let filename = &args[1];
    // let filename = "test.xenx";
    println!("Reading file: {}", filename);
    let context = std::fs::read_to_string(filename).expect("Unable to read file");
    let mut lexer = lexer::Lexer::new(context.clone());
    let tokens = lexer.lex();
    let mut parser = parser::Parser::new(tokens);
    let statements = parser.parse();
    let mut syntactic_analyser = syntactic_analyser::SyntaticAnalyser::new(statements, context.clone());
    let _statements = syntactic_analyser.analyse();
    println!("Statements: {}", _statements.clone().len());
    // from here i can use _statements to generate code
}
#[allow(dead_code)]
fn export_arguments(mut args: Vec<String>) -> (HashMap<String, String>, Vec<String>) {
    let mut map = HashMap::new();
    let mut rem_vec = Vec::<usize>::new();
    for(i, arg) in args.clone().iter().enumerate() {
        if i >= args.len() {
            break;
        }
        if arg.starts_with("--") {
            let key = arg.replace("-", "");
            rem_vec.push(i);
            let key = key+":option";
            map.insert(key, "".to_string());
        }
        else
        if arg.starts_with("-") {
            let key = arg.replace("-", "");
            rem_vec.push(i);
            if i+1 >= args.len() {
                let value = "".to_string();
                map.insert(key, value);
                break;
            }
            let value = args[i+1].clone();
            rem_vec.push(i+1);
            map.insert(key, value);
        }
    }
    let mut off = 0;
    for i in rem_vec {
        args.remove(i-off);
        off += 1;
    }
    return (map, args);
}