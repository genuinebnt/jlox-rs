#[derive(Debug, PartialEq)]
pub enum Token {
    LeftParen(char, usize),
    RightParen(char, usize),
    LeftBrace(char, usize),
    RightBrace(char, usize),
    Comma(char, usize),
    Dot(char, usize),
    Minus(char, usize),
    Plus(char, usize),
    Semicolon(char, usize),
    Slash(char, usize),
    Star(char, usize),

    // One or two character tokens.
    Bang(char, usize),
    BangEqual(String, usize),
    Equal(char, usize),
    EqualEqual(String, usize),
    Greater(String, usize),
    GreaterEqual(String, usize),
    Less(String, usize),
    LessEqual(String, usize),

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,

    Invalid(char, usize),
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Identifier(String),
    Str(String),
    Number(i64),
}
