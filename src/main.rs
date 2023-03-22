use std::env;
use std::fs;

use std::io::Write;
use std::io::{stdin, stdout};

fn run_file(path: &str) {
    let bytes = fs::read(path).unwrap();
}

fn run_prompt() {
    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();

        println!("{}", line);

        if line.trim() == "exit" {
            break;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage jlox [script]");
    } else if args.len() == 2 {
        run_file(&args[0]);
    } else {
        run_prompt();
    }
}
