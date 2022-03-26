use std::{env, fs, process, io};
use std::io::Write;
use exitcode;
use std::sync::Mutex;
use once_cell::sync::Lazy;

static HAD_ERROR_MUTEX: Lazy<Mutex<bool>> =  Lazy::new(|| {Mutex::new(false)});

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        print!("Usage: Coke [script]");
        process::exit(exitcode::USAGE);
    } else if args.len() == 2 {
        // run from the source file
        run_from_file(&args[0]);
    } else {
        // Enter the interpreter prompt
        run_prompt();
    }
}

/// Load the file into the memory and run it
fn run_from_file(file_name: &str) {
    let coke_source: String  = fs::read_to_string(file_name).expect("Unable to load the Coke source file");
    run(coke_source);
}

// Prompt mode, run the source in a repl mode, line by line
fn run_prompt() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    println!("Interactive Coke 0.1.0");
    loop {
        print!("icoke> ");
        io::stdout().flush().unwrap();
        match stdin.read_line(&mut buffer) {
            Ok(_source) => {
                if buffer.trim_end().is_empty() {
                    break;
                } 
                run(String::from(buffer.trim_end()));
                *HAD_ERROR_MUTEX.lock().unwrap() = false;
            }
            Err(error) => println!("Error due to {error:?}")
        }
    }
}

fn run(coke_source: String) {
    // Give the source to scanner module's scan function
    // It returns a list of Tokens
    // We will print the tokens
    if *HAD_ERROR_MUTEX.lock().unwrap() {
        process::exit(exitcode::DATAERR);
    }
}

fn report(line: i32, at: &str, message: &str) {
    let err = format!("[line {} ] Error {} : {}", line, at, message);
    *HAD_ERROR_MUTEX.lock().unwrap() = true;
}

fn error(line: i32, message: &str) {
    report(line, "", message)
}


