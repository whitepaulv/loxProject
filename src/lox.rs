mod tokenType;
mod token;
mod scanner;
mod expression;
mod astPrinter;
mod parser;

use scanner::Scanner;
use std::env;
use std::io::{self, Write}; // Write allows .flush() to be used


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        panic!("Error: needs arguments");
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
        panic!("There was an error");
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
    for token in tokens {
        println!("{}", token.to_string());
    }
}

fn error(had_error: &mut bool, line: usize, message: &str) {
    report(had_error, line, "", message);
}

fn report(had_error: &mut bool, line: usize, location: &str, message: &str) {
    println!("[line {line}] Error{location}: {message}");
    *had_error = true;
}