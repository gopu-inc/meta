use crate::ast::{Expr, Stmt};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Bool(bool),
    Function { params: Vec<String>, body: Vec<Stmt> },
    None,
}

#[derive(Debug, Clone)]
pub struct Env {
    pub vars: HashMap<String, Value>,
}

impl Env {
    pub fn new() -> Self {
        Env { vars: HashMap::new() }
    }

    pub fn eval_expr(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::Int(n) => Value::Int(*n),
            Expr::Bool(b) => Value::Bool(*b),
            Expr::Var(name) => self.vars.get(name).cloned().unwrap_or(Value::None),

            Expr::BinOp { left, op, right } => {
                let l = self.eval_expr(left);
                let r = self.eval_expr(right);
                match (l, r, op.as_str()) {
                    (Value::Int(a), Value::Int(b), "+") => Value::Int(a + b),
                    (Value::Int(a), Value::Int(b), "-") => Value::Int(a - b),
                    (Value::Int(a), Value::Int(b), "*") => Value::Int(a * b),
                    (Value::Int(a), Value::Int(b), "/") => Value::Int(a / b),
                    (Value::Int(a), Value::Int(b), "==") => Value::Bool(a == b),
                    (Value::Int(a), Value::Int(b), "!=") => Value::Bool(a != b),
                    (Value::Int(a), Value::Int(b), "<") => Value::Bool(a < b),
                    (Value::Int(a), Value::Int(b), ">") => Value::Bool(a > b),
                    _ => Value::None,
                }
            }

            Expr::Call { name, args } => {
                // On clone la fonction pour éviter le conflit d’emprunts
                let func = match self.vars.get(name) {
                    Some(Value::Function { params, body }) => (params.clone(), body.clone()),
                    _ => return Value::None,
                };

                let (params, body) = func;
                let mut local = Env::new();

                for (p, a) in params.iter().zip(args.iter()) {
                    let val = self.eval_expr(a);
                    local.vars.insert(p.clone(), val);
                }

                local.eval_block(&body)
            }
        }
    }

    pub fn eval_stmt(&mut self, stmt: &Stmt) -> Value {
        match stmt {
            Stmt::Let { name, expr } => {
                let val = self.eval_expr(expr);
                self.vars.insert(name.clone(), val.clone());
                val
            }

            Stmt::Expr(expr) => self.eval_expr(expr),

            Stmt::Fn { name, params, body } => {
                self.vars.insert(
                    name.clone(),
                    Value::Function {
                        params: params.clone(),
                        body: body.clone(),
                    },
                );
                Value::None
            }

            Stmt::If { cond, then_branch, else_branch } => {
                match self.eval_expr(cond) {
                    Value::Bool(true) => self.eval_block(then_branch),
                    Value::Bool(false) => self.eval_block(else_branch),
                    _ => Value::None,
                }
            }

            Stmt::Return(expr) => self.eval_expr(expr),
        }
    }

    pub fn eval_block(&mut self, block: &[Stmt]) -> Value {
        let mut last = Value::None;
        for stmt in block {
            last = self.eval_stmt(stmt);
        }
        last
    }
}
