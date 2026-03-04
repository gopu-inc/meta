// src/ast.rs

#[derive(Debug, Clone)]
pub enum Stmt {
    // Déclaration d'une variable : let name = expr;
    Let {
        name: String,
        expr: Expr,
    },
    // Expression seule : e.g. a + b;
    Expr(Expr),
}

#[derive(Debug, Clone)]
pub enum Expr {
    // Nombre entier
    Number(i64),
    // Variable
    Variable(String),
    // Expression binaire : left op right
    Binary {
        left: Box<Expr>,
        op: String,
        right: Box<Expr>,
    },
    // Parenthèse ou regroupement
    Grouping(Box<Expr>),
}

// Pour les opérateurs binaires, tu peux définir une enum si tu veux plus de sécurité :
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    pub fn from_str(s: &str) -> Option<Op> {
        match s {
            "+" => Some(Op::Add),
            "-" => Some(Op::Sub),
            "*" => Some(Op::Mul),
            "/" => Some(Op::Div),
            _ => None,
        }
    }
}
