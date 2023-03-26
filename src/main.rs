use std::env;
use std::process::exit;

mod lox;
mod scanner;
mod token;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: jlox-rs [script]");
        exit(0);
    } else if args.len() == 1 {
        todo!()
    } else {
        todo!()
    }
}
