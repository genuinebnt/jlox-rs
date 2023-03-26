use core::fmt;
use std::fs;
use std::io::Write;
use std::io::{self};

use crate::scanner::Scanner;
use crate::token::Token;

type Result<T> = std::result::Result<T, LoxError>;

#[derive(Debug, PartialEq)]
enum LoxError {
    ReadError,
    PromptReadError,
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoxError::ReadError => write!(f, "Cannot read contents of file"),
            LoxError::PromptReadError => write!(f, "Cannot read from stdin"),
        }
    }
}

struct Lox {}

impl Lox {
    fn new() -> Self {
        Lox {}
    }

    fn run_file(path: &str) -> Result<()> {
        let contents = fs::read_to_string(path).map_err(|_| LoxError::ReadError)?;

        Self::run(&contents);

        Ok(())
    }

    fn run_prompt() -> Result<()> {
        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .map_err(|_| LoxError::PromptReadError)?;

            println!("{input}");

            if input.trim() == "exit" {
                break;
            }
        }

        Ok(())
    }

    fn run(contents: &str) {
        let mut scanner = Scanner::new(contents.to_string());
        let tokens: &Vec<Token> = &scanner.scan_tokens_iter().unwrap();

        tokens.iter().for_each(|token| println! {"{:?}", token})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_prompt() {
        Lox::run_prompt().unwrap();
    }

    fn test_scanner() {
        let contents = "     }}}} ) )
                ) }(,
                        . ***";

        Lox::run(contents);
    }
}
