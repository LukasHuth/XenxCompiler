use std::collections::HashMap;
mod lexer;
mod parser;
// https://norasandler.com/2017/11/29/Write-a-Compiler.html
fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    // let dash_args = export_arguments(args);
    // let args = dash_args.clone().1;
    // let dash_args = dash_args.clone().0;
    // println!("Arguments: {:?}", dash_args);
    // println!("Arguments: {:?}", args);
    if args.len() < 2
    {
        println!("Usage: {} <input>", args[0]);
    }
    let filename = &args[1];
    println!("Reading file: {}", filename);
    // let filename = "../../../".to_string()+filename;
    let context = std::fs::read_to_string(filename).expect("Unable to read file");
    let mut lexer = lexer::Lexer::new(context.clone());
    let tokens = lexer.lex();
    // for token in tokens
    // {
    //     println!("{}", token.to_string());
    // }
    let mut parser = parser::Parser::new(tokens);
    let statements = parser.parse();
    println!("statements: {}", statements.len());
    for statement in statements
    {
        println!("{}", statement.to_string());
    }
    // println!("Hello, world!");
}
#[allow(dead_code)]
fn export_arguments(mut args: Vec<String>) -> (HashMap<String, String>, Vec<String>) {
    // println!("Arguments: {:?}", args);
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