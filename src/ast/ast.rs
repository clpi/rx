
pub type BExpr = Box<Expr>;
/// The AST node for expressions.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Expr {
    Literal(String),
    Identifier(String),
    Pow(BExpr, BExpr),
    Assign(String, BExpr),
    Eq(BExpr, BExpr),
    Not(BExpr),
    Ne(BExpr, BExpr),
    Lt(BExpr, BExpr),
    Le(BExpr, BExpr),
    Gt(BExpr, BExpr),
    Ge(BExpr, BExpr),
    Add(BExpr, BExpr),
    Sub(BExpr, BExpr),
    Mul(BExpr, BExpr),
    Div(BExpr, BExpr),
    And(BExpr, BExpr),
    Or(BExpr, BExpr),
    Xor(BExpr, BExpr),
    IfElse(BExpr, Vec<Expr>, Vec<Expr>),
    WhileLoop(BExpr, Vec<Expr>),
    Call(String, Vec<Expr>),
    GlobalDataAddr(String),
}


impl Expr {
    pub fn ident(e: String) -> Self {
        Expr::Identifier(e)
    }
    pub fn literal(e: String) -> Self {
        Expr::Literal(e)
    }
    pub fn not(e: Expr) -> Self {
        Expr::Not(Box::new(e))
    }
}
