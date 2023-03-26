use std::fmt::Display;

use crate::token::Token;

type Result<T> = std::result::Result<T, ScannerError>;

#[derive(Debug)]
pub enum ScannerError {
    UnexpectedToken(char, usize),
}

impl Display for ScannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScannerError::UnexpectedToken(e, pos) => {
                write!(f, "Unexpected token {} at position {}", e, pos)
            }
        }
    }
}

#[derive(Debug)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
        }
    }

    pub fn scan_tokens_iter(&mut self) -> Result<&Vec<Token>> {
        let mut char_indices = self
            .source
            .char_indices()
            .peekable()
            .filter(|(pos, c)| !c.is_ascii_whitespace());

        while let Some((pos, c)) = char_indices.next() {
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
                _ => Err(ScannerError::UnexpectedToken(c, pos)),
            };

            self.tokens.push(token?);
        }

        Ok(&self.tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_scan_tokens_iter() {
        let contents = "+++
        ++}}   
                }---";

        let mut scanner = Scanner::new(contents.to_string());
        println!("{:?}", scanner);

        println!("{:?}", scanner.scan_tokens_iter());
    }
}
