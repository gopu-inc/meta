#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Int(i64),
    Bool(bool),
    Ident(String),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Eq,
    Semicolon,
    If,
    Else,
    Fn,
    Let,
    Return,
    Lt,
    Gt,
    EqEq,
    NotEq,
    EOF,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer { input: input.chars().collect(), pos: 0 }
    }

    fn next_char(&mut self) -> Option<char> {
        if self.pos >= self.input.len() { None } else { Some(self.input[self.pos]) }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(ch) = self.next_char() {
            match ch {
                '0'..='9' => {
                    let mut num = String::new();
                    while let Some(d) = self.next_char() {
                        if d.is_digit(10) {
                            num.push(d);
                            self.pos += 1;
                        } else { break; }
                    }
                    tokens.push(Token::Int(num.parse().unwrap()));
                    continue;
                }
                '+' => tokens.push(Token::Plus),
                '-' => tokens.push(Token::Minus),
                '*' => tokens.push(Token::Star),
                '/' => tokens.push(Token::Slash),
                '(' => tokens.push(Token::LParen),
                ')' => tokens.push(Token::RParen),
                '{' => tokens.push(Token::LBrace),
                '}' => tokens.push(Token::RBrace),
                ',' => tokens.push(Token::Comma),
                ';' => tokens.push(Token::Semicolon),
                '=' => {
                    self.pos += 1;
                    if self.next_char() == Some('=') { tokens.push(Token::EqEq) } 
                    else { tokens.push(Token::Eq); continue; }
                }
                '<' => tokens.push(Token::Lt),
                '>' => tokens.push(Token::Gt),
                '!' => {
                    self.pos += 1;
                    if self.next_char() == Some('=') { tokens.push(Token::NotEq) }
                }
                c if c.is_alphabetic() => {
                    let mut ident = String::new();
                    while let Some(a) = self.next_char() {
                        if a.is_alphanumeric() { ident.push(a); self.pos += 1; }
                        else { break; }
                    }
                    let token = match ident.as_str() {
                        "if" => Token::If,
                        "else" => Token::Else,
                        "fn" => Token::Fn,
                        "let" => Token::Let,
                        "return" => Token::Return,
                        "true" => Token::Bool(true),
                        "false" => Token::Bool(false),
                        _ => Token::Ident(ident),
                    };
                    tokens.push(token);
                    continue;
                }
                c if c.is_whitespace() => {},
                _ => panic!("Unknown character: {}", ch),
            }
            self.pos += 1;
        }
        tokens.push(Token::EOF);
        tokens
    }
}
