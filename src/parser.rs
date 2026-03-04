use crate::lexer::Token;
use crate::ast::{Expr, Stmt};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, position: 0 }
    }

    fn current(&self) -> Token {
        self.tokens
            .get(self.position)
            .cloned()
            .unwrap_or(Token::EOF)
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn eat(&mut self, expected: Token) {
        if self.current() == expected {
            self.advance();
        } else {
            panic!("Expected {:?}, found {:?}", expected, self.current());
        }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while self.current() != Token::EOF {
            statements.push(self.parse_stmt());
        }
        statements
    }

    fn parse_stmt(&mut self) -> Stmt {
        match self.current() {
            Token::Let => {
                self.advance();
                if let Token::Ident(name) = self.current() {
                    self.advance();
                    self.eat(Token::Equal);
                    let expr = self.parse_expr();
                    self.eat(Token::Semicolon);
                    Stmt::Let(name, expr)
                } else {
                    panic!("Expected identifier after let");
                }
            }
            Token::Return => {
                self.advance();
                let expr = self.parse_expr();
                self.eat(Token::Semicolon);
                Stmt::Return(expr)
            }
            _ => {
                let expr = self.parse_expr();
                self.eat(Token::Semicolon);
                Stmt::Expr(expr)
            }
        }
    }

    fn parse_expr(&mut self) -> Expr {
        self.parse_equality()
    }

    fn parse_equality(&mut self) -> Expr {
        let mut expr = self.parse_comparison();

        while self.current() == Token::EqualEqual
            || self.current() == Token::BangEqual
        {
            let op = self.current();
            self.advance();
            let right = self.parse_comparison();

            let op_str = match op {
                Token::EqualEqual => "==",
                Token::BangEqual => "!=",
                _ => unreachable!(),
            };

            expr = Expr::Binary(Box::new(expr), op_str.to_string(), Box::new(right));
        }

        expr
    }

    fn parse_comparison(&mut self) -> Expr {
        let mut expr = self.parse_term();

        while self.current() == Token::Less
            || self.current() == Token::Greater
            || self.current() == Token::LessEqual
            || self.current() == Token::GreaterEqual
        {
            let op = self.current();
            self.advance();
            let right = self.parse_term();

            let op_str = match op {
                Token::Less => "<",
                Token::Greater => ">",
                Token::LessEqual => "<=",
                Token::GreaterEqual => ">=",
                _ => unreachable!(),
            };

            expr = Expr::Binary(Box::new(expr), op_str.to_string(), Box::new(right));
        }

        expr
    }

    fn parse_term(&mut self) -> Expr {
        let mut expr = self.parse_factor();

        while self.current() == Token::Plus
            || self.current() == Token::Minus
        {
            let op = self.current();
            self.advance();
            let right = self.parse_factor();

            let op_str = match op {
                Token::Plus => "+",
                Token::Minus => "-",
                _ => unreachable!(),
            };

            expr = Expr::Binary(Box::new(expr), op_str.to_string(), Box::new(right));
        }

        expr
    }

    fn parse_factor(&mut self) -> Expr {
        let mut expr = self.parse_primary();

        while self.current() == Token::Star
            || self.current() == Token::Slash
        {
            let op = self.current();
            self.advance();
            let right = self.parse_primary();

            let op_str = match op {
                Token::Star => "*",
                Token::Slash => "/",
                _ => unreachable!(),
            };

            expr = Expr::Binary(Box::new(expr), op_str.to_string(), Box::new(right));
        }

        expr
    }

    fn parse_primary(&mut self) -> Expr {
        match self.current() {
            Token::Number(n) => {
                self.advance();
                Expr::Number(n)
            }
            Token::Ident(ref name) => {
                let name = name.clone();
                self.advance();
                Expr::Variable(name)
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expr();
                self.eat(Token::RParen);
                expr
            }
            _ => panic!("Unexpected token: {:?}", self.current()),
        }
    }
            }
