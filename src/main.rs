use std::collections::HashMap;
pub mod lexer;
pub mod parser;
pub mod syntactic_analyser;
pub mod codegen;
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
    let outfile: String;
    {
        outfile = if dash_args.contains_key("o") {
            dash_args.get("o").unwrap().clone()
        } else {
            "out".to_string()
        };
    }
    println!("outfile: {}", outfile);
    let comments = if dash_args.contains_key("c") {
        true
    } else {
        false
    };
    let os: codegen::OS;
    {
        os = if dash_args.contains_key("f") {
            let os = dash_args.get("f").unwrap().clone();
            if os == "elf" {
                codegen::OS::Linux
            } else {
                panic!("OS not supported");
            }
        } else {
            codegen::OS::Linux
        }
    }
    // let filename = "test.xenx";
    println!("Reading file: {}", filename);
    let context = std::fs::read_to_string(filename).expect("Unable to read file");
    // let mut lexer = lexer::Lexer::new(context.clone());
    // let tokens = lexer.lex();
    // let mut parser = parser::Parser::new(tokens);
    // let statements = parser.parse();
    let parsed_file = parse_file(filename);
    let statements = parsed_file.0;
    let include_std = parsed_file.1;
    let mut syntactic_analyser = syntactic_analyser::SyntaticAnalyser::new(statements, context.clone(), include_std);
    let _statements_function_tuple = syntactic_analyser.analyse();
    // println!("Statements: {}", _statements.clone().len());
    let functions = _statements_function_tuple.1;
    let _statements = _statements_function_tuple.0;
    let std_functions = syntactic_analyser.get_std_functions();
    let mut codegen = codegen::Codegen::new(_statements,functions, os, std_functions, comments);
    codegen.generate();
    codegen.compile(outfile.as_str());
    // from here i can use _statements to generate code
}
fn parse_file(filename: &str) -> (Vec<parser::expression::Expression>, bool) {
    let mut tokens = get_tokens_from_file(filename);
    let mut filenames = Vec::<String>::new();
    let mut statements = Vec::<parser::expression::Expression>::new();
    let mut include_std = false;
    while tokens[0].token == lexer::token::LexerToken::Keyword && tokens[0].text == "import" {
        tokens.drain(0..1);
        let mut filename = String::new();
        while tokens[0].token != lexer::token::LexerToken::Semicolon {
            filename.push_str(tokens[0].text.as_str());
            tokens.drain(0..1);
        }
        filenames.push(filename);
    }
    for filename in filenames {
        if filename == "std" {
            include_std = true;
            continue;
        }
        let res = parse_file(filename.as_str());
        if res.1 { include_std = true; }
        let mut res = res.0;
        statements.append(&mut res);
    }
    let mut parser = parser::Parser::new(tokens);
    statements.append(&mut parser.parse());
    return ( statements, include_std );
}
fn get_tokens_from_file(filename: &str) -> Vec<lexer::token::Token> {
    let context = std::fs::read_to_string(filename).expect("Unable to read file");
    let mut lexer = lexer::Lexer::new(context.clone());
    let tokens = lexer.lex();
    return tokens;
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