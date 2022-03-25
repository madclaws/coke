use std::{env, fs, process, io};
use std::io::Write;
use exitcode;
fn main() {
    println!("Coke 0.1.0");
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
    loop {
        print!("Coke => ");
        io::stdout().flush().unwrap();
        match stdin.read_line(&mut buffer) {
            Ok(_source) => 
            println!("{:?}", buffer.trim_end()),
            Err(error) => println!("Error due to {error:?}")
        }
    }
}

fn run(coke_source: String) {

}
