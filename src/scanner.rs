use std::{borrow::Borrow, fmt::Display};

use crate::token::Token;

type Result<T> = std::result::Result<T, ScannerError>;

#[derive(Debug)]
pub enum ScannerError {
    UnexpectedToken(char, usize),
    UnterminatedString(String, usize),
}

impl Display for ScannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScannerError::UnexpectedToken(e, pos) => {
                write!(f, "Unexpected token {} at position {}", e, pos)
            }
            ScannerError::UnterminatedString(s, line) => {
                write!(f, "Unterminated string {} at line {}", s, line)
            }
        }
    }
}

#[derive(Debug)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            line: 1,
        }
    }

    pub fn scan_tokens_iter(&mut self) -> Result<&Vec<Token>> {
        let mut char_indice = self.source.char_indices().peekable();

        while let Some((pos, c)) = char_indice.next() {
            let token = match c {
                '(' => Ok(Token::LeftParen(c, pos)),
                ')' => Ok(Token::RightParen(c, pos)),
                '{' => Ok(Token::LeftBrace(c, pos)),
                '}' => Ok(Token::RightBrace(c, pos)),
                ',' => Ok(Token::Comma(c, pos)),
                '.' => Ok(Token::Dot(c, pos)),
                '-' => Ok(Token::Minus(c, pos)),
                '+' => Ok(Token::Plus(c, pos)),
                ';' => Ok(Token::Semicolon(c, pos)),
                '*' => Ok(Token::Star(c, pos)),
                '!' => match char_indice.next_if_eq(&(pos + 1, '=')) {
                    Some(_) => Ok(Token::BangEqual("!=".to_string(), pos)),
                    None => Ok(Token::Bang('!', pos)),
                },
                '=' => match char_indice.next_if_eq(&(pos + 1, '=')) {
                    Some(_) => Ok(Token::EqualEqual("==".to_string(), pos)),
                    None => Ok(Token::Equal('=', pos)),
                },
                '<' => match char_indice.next_if_eq(&(pos + 1, '=')) {
                    Some(_) => Ok(Token::LessEqual("<=".to_string(), pos)),
                    None => Ok(Token::Less('<', pos)),
                },
                '>' => match char_indice.next_if_eq(&(pos + 1, '=')) {
                    Some(_) => Ok(Token::GreaterEqual(">=".to_string(), pos)),
                    None => Ok(Token::Greater('>', pos)),
                },
                '/' => match char_indice.next_if_eq(&(pos + 1, '/')) {
                    Some(_) => {
                        let _comment: String = char_indice
                            .by_ref()
                            .take_while(|(_pos, ch)| *ch != '\n')
                            .map(|(_pos, ch)| ch)
                            .collect();

                        self.line += 1;
                        Ok(Token::Comment)
                    }
                    None => Ok(Token::Slash(c, pos)),
                },
                '\r' | '\t' | ' ' => Ok(Token::Skip),
                '\n' => {
                    self.line += 1;
                    Ok(Token::Skip)
                }
                '"' => {
                    let mut last_char: char = char::default();
                    let string_literal: String = char_indice
                        .by_ref()
                        .take_while(|(_pos, ch)| {
                            if *ch == '\n' {
                                self.line += 1
                            };
                            last_char = *ch;
                            *ch != '"'
                        })
                        .map(|(_pos, ch)| ch)
                        .collect();

                    match last_char {
                        '"' => Ok(Token::String(string_literal)),
                        _ => Err(ScannerError::UnterminatedString(string_literal, pos)),
                    }
                }
                c if c.is_numeric() => {
                    let digit = char_indice
                        .by_ref()
                        .take_while(|(_pos, ch)| {
                            if ch.is_numeric() {
                                true
                            } else {
                                match char_indice.peek() {
                                    Some((_pos, next_ch)) => *ch == '.' && next_ch.is_numeric(),
                                    None => false,
                                }
                            }
                        })
                        .map(|(_pos, ch)| ch)
                        .collect::<String>();

                    let digit = digit.parse::<i64>().unwrap();

                    Ok(Token::Number(digit))
                }
                _ => Err(ScannerError::UnexpectedToken(c, self.line)),
            };

            match token {
                Ok(t) => match t {
                    Token::Comment | Token::Skip => {}
                    _ => self.tokens.push(t),
                },
                Err(e) => return Err(e),
            }
        }

        Ok(&self.tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_scan_tokens_iter() {
        let contents = "+++++}}/}->===<=--//this is a comment
            //another comment
            //and another one
            \"a new string";

        let mut scanner = Scanner::new(contents.to_string());
        println!("{:?}", scanner);

        println!("{:?}", scanner.scan_tokens_iter());
    }
}
