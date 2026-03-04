#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Number(i64),
    Ident(String),

    // Keywords
    Fn,
    Let,
    If,
    Else,
    Return,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // Symbols
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Semicolon,

    EOF,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn current(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn skip_comment(&mut self) {
        while let Some(c) = self.current() {
            if c == '\n' {
                break;
            }
            self.advance();
        }
    }

    fn read_number(&mut self) -> i64 {
        let mut num = String::new();
        while let Some(c) = self.current() {
            if c.is_ascii_digit() {
                num.push(c);
                self.advance();
            } else {
                break;
            }
        }
        num.parse().unwrap()
    }

    fn read_ident(&mut self) -> String {
        let mut ident = String::new();
        while let Some(c) = self.current() {
            if c.is_alphanumeric() || c == '_' {
                ident.push(c);
                self.advance();
            } else {
                break;
            }
        }
        ident
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.current() {
            Some(c) => match c {
                '+' => {
                    self.advance();
                    Token::Plus
                }
                '-' => {
                    self.advance();
                    Token::Minus
                }
                '*' => {
                    self.advance();
                    Token::Star
                }
                '/' => {
                    self.advance();
                    if self.current() == Some('/') {
                        self.advance();
                        self.skip_comment();
                        self.next_token()
                    } else {
                        Token::Slash
                    }
                }
                '=' => {
                    self.advance();
                    if self.current() == Some('=') {
                        self.advance();
                        Token::EqualEqual
                    } else {
                        Token::Equal
                    }
                }
                '!' => {
                    self.advance();
                    if self.current() == Some('=') {
                        self.advance();
                        Token::BangEqual
                    } else {
                        Token::Bang
                    }
                }
                '<' => {
                    self.advance();
                    if self.current() == Some('=') {
                        self.advance();
                        Token::LessEqual
                    } else {
                        Token::Less
                    }
                }
                '>' => {
                    self.advance();
                    if self.current() == Some('=') {
                        self.advance();
                        Token::GreaterEqual
                    } else {
                        Token::Greater
                    }
                }
                '(' => {
                    self.advance();
                    Token::LParen
                }
                ')' => {
                    self.advance();
                    Token::RParen
                }
                '{' => {
                    self.advance();
                    Token::LBrace
                }
                '}' => {
                    self.advance();
                    Token::RBrace
                }
                ',' => {
                    self.advance();
                    Token::Comma
                }
                ';' => {
                    self.advance();
                    Token::Semicolon
                }
                c if c.is_ascii_digit() => {
                    let number = self.read_number();
                    Token::Number(number)
                }
                c if c.is_alphabetic() || c == '_' => {
                    let ident = self.read_ident();
                    match ident.as_str() {
                        "fn" => Token::Fn,
                        "let" => Token::Let,
                        "if" => Token::If,
                        "else" => Token::Else,
                        "return" => Token::Return,
                        _ => Token::Ident(ident),
                    }
                }
                _ => {
                    panic!("Caractère inattendu: {}", c);
                }
            },
            None => Token::EOF,
        }
    }
                }
