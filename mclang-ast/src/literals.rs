use crate::{expression::Expr, Ident};

#[derive(Debug, PartialEq)]
pub struct ArrayLiteral {
    pub items: Vec<Expr>
}

#[derive(Debug, PartialEq)]
pub struct StructLiteral {
    pub items: Vec<(Ident, Expr)>
}