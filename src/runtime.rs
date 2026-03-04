use std::collections::HashMap;
use crate::ast::{Expr, Stmt};

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Bool(bool),
    None,
}

pub struct Runtime {
    pub globals: HashMap<String, Value>,
    pub functions: HashMap<String, (Vec<String>, Vec<Stmt>)>,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            globals: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn run(&mut self, statements: Vec<Stmt>) -> Value {
        let mut last = Value::None;
        for stmt in statements {
            last = self.exec_stmt(stmt);
        }
        last
    }

    fn exec_stmt(&mut self, stmt: Stmt) -> Value {
        match stmt {
            Stmt::Let(name, expr) => {
                let val = self.eval_expr(expr);
                self.globals.insert(name, val.clone());
                val
            }
            Stmt::Expr(expr) => self.eval_expr(expr),
            Stmt::Return(expr) => self.eval_expr(expr),
            Stmt::Fn { name, params, body } => {
                self.functions.insert(name, (params, body));
                Value::None
            }
            Stmt::If { cond, then_branch, else_branch } => {
                let cond_val = self.eval_expr(cond);
                match cond_val {
                    Value::Bool(true) => self.run(then_branch),
                    Value::Bool(false) => self.run(else_branch),
                    _ => panic!("Condition must be a boolean"),
                }
            }
        }
    }

    fn eval_expr(&mut self, expr: Expr) -> Value {
        match expr {
            Expr::Number(n) => Value::Int(n),
            Expr::Bool(b) => Value::Bool(b),
            Expr::Variable(name) => {
                self.globals.get(&name).cloned().unwrap_or(Value::None)
            }
            Expr::Binary(left, op, right) => {
                let l = self.eval_expr(*left);
                let r = self.eval_expr(*right);
                self.eval_binary(op, l, r)
            }
            Expr::Call { name, args } => {
                if let Some((params, body)) = self.functions.get(&name).cloned() {
                    if params.len() != args.len() {
                        panic!("Fonction {}: mauvais nombre d'arguments", name);
                    }

                    // Scope temporaire pour la fonction
                    let mut backup = self.globals.clone();
                    for (p, a) in params.iter().zip(args) {
                        let val = self.eval_expr(a);
                        self.globals.insert(p.clone(), val);
                    }

                    let result = self.run(body);
                    self.globals = backup;
                    result
                } else {
                    panic!("Fonction non définie: {}", name);
                }
            }
        }
    }

    fn eval_binary(&self, op: String, l: Value, r: Value) -> Value {
        match (l, r) {
            (Value::Int(a), Value::Int(b)) => match op.as_str() {
                "+" => Value::Int(a + b),
                "-" => Value::Int(a - b),
                "*" => Value::Int(a * b),
                "/" => Value::Int(a / b),
                "==" => Value::Bool(a == b),
                "!=" => Value::Bool(a != b),
                "<" => Value::Bool(a < b),
                "<=" => Value::Bool(a <= b),
                ">" => Value::Bool(a > b),
                ">=" => Value::Bool(a >= b),
                _ => panic!("Opérateur inconnu {}", op),
            },
            (Value::Bool(a), Value::Bool(b)) => match op.as_str() {
                "==" => Value::Bool(a == b),
                "!=" => Value::Bool(a != b),
                _ => panic!("Opérateur booléen inconnu {}", op),
            },
            _ => panic!("Types incompatibles pour l'opérateur {}", op),
        }
    }
}
