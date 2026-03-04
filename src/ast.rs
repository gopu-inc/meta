#[derive(Debug, Clone)]
pub enum Expr {
    Int(i64),
    Bool(bool),
    Var(String),
    BinOp { left: Box<Expr>, op: String, right: Box<Expr> },
    Call { name: String, args: Vec<Expr> },
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let { name: String, expr: Expr },
    Return(Expr),
    Expr(Expr),
    If { cond: Expr, then_branch: Vec<Stmt>, else_branch: Vec<Stmt> },
    Fn { name: String, params: Vec<String>, body: Vec<Stmt> },
}
