use crate::lexer::Token;
use crate::ast::{Expr, Stmt};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn current(&self) -> Token {
        self.tokens.get(self.pos).cloned().unwrap_or(Token::EOF)
    }

    fn advance(&mut self) {
        self.pos += 1;
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
                    self.eat(Token::Eq);
                    let expr = self.parse_expr();
                    self.eat(Token::Semicolon);
                    Stmt::Let { name, expr }
                } else {
                    panic!("Expected identifier after let");
                }
            }

            Token::Fn => {
                self.advance();
                if let Token::Ident(name) = self.current() {
                    self.advance();

                    self.eat(Token::LParen);
                    let mut params = Vec::new();
                    while let Token::Ident(p) = self.current() {
                        params.push(p);
                        self.advance();
                        if self.current() == Token::Comma {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    self.eat(Token::RParen);

                    self.eat(Token::LBrace);
                    let mut body = Vec::new();
                    while self.current() != Token::RBrace {
                        body.push(self.parse_stmt());
                    }
                    self.eat(Token::RBrace);

                    Stmt::Fn { name, params, body }
                } else {
                    panic!("Expected function name");
                }
            }

            Token::If => {
                self.advance();
                let condition = self.parse_expr();

                self.eat(Token::LBrace);
                let mut then_branch = Vec::new();
                while self.current() != Token::RBrace {
                    then_branch.push(self.parse_stmt());
                }
                self.eat(Token::RBrace);

                let mut else_branch = Vec::new();
                if self.current() == Token::Else {
                    self.advance();
                    self.eat(Token::LBrace);
                    while self.current() != Token::RBrace {
                        else_branch.push(self.parse_stmt());
                    }
                    self.eat(Token::RBrace);
                }

                Stmt::If {
                    cond: condition,
                    then_branch,
                    else_branch,
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

    // ===============================
    // EXPRESSIONS (avec priorité)
    // ===============================

    fn parse_expr(&mut self) -> Expr {
        self.parse_equality()
    }

    fn parse_equality(&mut self) -> Expr {
        let mut left = self.parse_comparison();

        while self.current() == Token::EqEq || self.current() == Token::NotEq {
            let op = match self.current() {
                Token::EqEq => "==",
                Token::NotEq => "!=",
                _ => unreachable!(),
            }.to_string();

            self.advance();
            let right = self.parse_comparison();
            left = Expr::BinOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_comparison(&mut self) -> Expr {
        let mut left = self.parse_term();

        while self.current() == Token::Lt || self.current() == Token::Gt {
            let op = match self.current() {
                Token::Lt => "<",
                Token::Gt => ">",
                _ => unreachable!(),
            }.to_string();

            self.advance();
            let right = self.parse_term();
            left = Expr::BinOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_term(&mut self) -> Expr {
        let mut left = self.parse_factor();

        while self.current() == Token::Plus || self.current() == Token::Minus {
            let op = match self.current() {
                Token::Plus => "+",
                Token::Minus => "-",
                _ => unreachable!(),
            }.to_string();

            self.advance();
            let right = self.parse_factor();
            left = Expr::BinOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_factor(&mut self) -> Expr {
        let mut left = self.parse_primary();

        while self.current() == Token::Star || self.current() == Token::Slash {
            let op = match self.current() {
                Token::Star => "*",
                Token::Slash => "/",
                _ => unreachable!(),
            }.to_string();

            self.advance();
            let right = self.parse_primary();
            left = Expr::BinOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_primary(&mut self) -> Expr {
        match self.current() {

            Token::Int(n) => {
                self.advance();
                Expr::Int(n)
            }

            Token::Bool(b) => {
                self.advance();
                Expr::Bool(b)
            }

            Token::Ident(ref name) => {
                let name = name.clone();
                self.advance();

                if self.current() == Token::LParen {
                    self.advance();
                    let mut args = Vec::new();

                    if self.current() != Token::RParen {
                        loop {
                            args.push(self.parse_expr());
                            if self.current() == Token::Comma {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                    }

                    self.eat(Token::RParen);

                    Expr::Call { name, args }
                } else {
                    Expr::Var(name)
                }
            }

            Token::LParen => {
                self.advance();
                let expr = self.parse_expr();
                self.eat(Token::RParen);
                expr
            }

            _ => panic!("Unexpected token {:?}", self.current()),
        }
    }
              }
