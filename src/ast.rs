#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    Variable(String),
    Binary {
        left: Box<Expr>,
        op: String,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let { name: String, expr: Expr },
    Expr(Expr),
}
