use once_cell::sync::Lazy;
use std::cmp::Ordering;
use std::io::Write;
use std::sync::Mutex;
use std::{env, fs, io, process};

mod scanner;
use scanner::*;
mod token;
use token::*;
mod token_type;
use token_type::*;
mod ast_printer;
mod expr;
mod money_parser;
mod parser;
use parser::*;
use expr::*;
use ast_printer::*;
static HAD_ERROR_MUTEX: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len().cmp(&2) {
        Ordering::Greater => {
            println!("Usage: Coke [script]");
            process::exit(exitcode::USAGE);
        }
        Ordering::Equal => {
            // run from the source file
            run_from_file(&args[0]);
        }
        _ => {
            // Enter the interpreter prompt
            run_prompt();
        }
    }
}

/// Load the file into the memory and run it
fn run_from_file(file_name: &str) {
    let coke_source: String =
        fs::read_to_string(file_name).expect("Unable to load the Coke source file");
    run(coke_source);
}

// Prompt mode, run the source in a repl mode, line by line
fn run_prompt() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    println!("Cosmos Oriented Kernel English 🚀");
    println!("Interactive Coke (0.1.0)");
    loop {
        print!("icoke> ");

        io::stdout().flush().unwrap();
        match stdin.read_line(&mut buffer) {
            Ok(_source) => {
                buffer = String::from(buffer.trim_end());
                if buffer.is_empty() {
                    println!("Live long and prosper 🖖");
                    break;
                }
                run(buffer);
                buffer = String::from("");
                *HAD_ERROR_MUTEX.lock().unwrap() = false;
            }
            Err(error) => println!("Error due to {error:?}"),
        }
    }
}

fn run(source: String) {
    let scanner: Scanner = Scanner::new(source);
    let tokens: Vec<Token> = scanner.scan_tokens();
    let parser = Parser::new(tokens);
    let expression = parser.parse_expression();
    match expression {
        Ok(expr) => {
            let mut ast_printer = AstPrinter{};
            let parse_result = ast_printer.visit_expr(&expr);
            println!("{parse_result:?}");
        },
        Err(err) => {
            println!("Error parsing due to {:?}", err);
            if *HAD_ERROR_MUTEX.lock().unwrap() {
                process::exit(exitcode::DATAERR);
            }
        }
    }
}

fn report(line: i32, at: &str, message: &str) {
    let err = format!("[line {} ] Error {} : {}", line, at, message);
    *HAD_ERROR_MUTEX.lock().unwrap() = true;
    println!("{err}");
}

#[allow(dead_code)]
pub fn error(line: i32, message: &str) {
    report(line, "", message)
}

#[allow(dead_code)]
pub fn errorv2(token: &Token, message: &str) {
    if token.token_type == TokenType::Eof {
        report(token.line as i32, " at end", message)
    } else {
        let err_msg = format!(" at '{}'", token.lexeme);
        report(token.line as i32, &err_msg, message)
    }
}
