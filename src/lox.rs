mod token_type;
mod token;
mod scanner;
mod expression;
mod ast;
mod parser;

use scanner::Scanner;
use ast::ast_printer;
use std::env;
use std::io::{self, Write}; // Write allows .flush() to be used
use parser::Parser;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        panic!("Error: too many arguments were provided");
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(path: &String) { 

    let mut had_error = false;

    let bytes: Vec<u8> = std::fs::read(path).expect("Could not read file");
    let source = String::from_utf8(bytes).expect("File is not valid UTF-8");
    run(&source, &mut had_error);

    if had_error {
        std::process::exit(65);
    }
}

fn run_prompt() {
    let stdin = io::stdin();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        let n = stdin.read_line(&mut line).expect("Could not read line");
        if n == 0 { // n is used to count how many bytes were read. If n == 0, line is empty* and we can return
            break;
        }

        let mut had_error = false;
        run(line.trim_end(), &mut had_error);
    }
}

fn run(source: &str, had_error: &mut bool) { // USE &str INSTEAD OF &string WHEREVER POSSIBLE!! not sure why but man it makes life easier. 
    let scanner = Scanner::new(source.to_string());
    let tokens = scanner.scan_tokens(had_error);

    let mut parser = Parser::new(tokens);
    let expr = parser.parse(had_error);
    if !*had_error {
    println!("{}", ast_printer(&expr));
    // FIXME: need to finish later
}
}

fn error(had_error: &mut bool, line: usize, message: &str) {
    report(had_error, line, "", message);
}

fn report(had_error: &mut bool, line: usize, location: &str, message: &str) {
    println!("[line {line}] Error{location}: {message}"); // Might want to change to eprintln! in the future? 
    *had_error = true;                                    // Not sure how the intended functionality is of the Lox book
}