use crate::{statement::Var, function::{FunctionDef, FunctionCall}, ops::{BinOp, Unop}, Ident, literals::{ArrayLiteral, StructLiteral}};


/// exp ::= nil | false | true | Numeral | LiteralString | '...' | functiondef |
///         prefixexp | tableconstructor | exp binop exp | unop exp
#[derive(Debug, PartialEq)]
pub enum Expr {
    Nil,
    Bool(bool),
    Num(f64),
    Str(String),
    Dots,
    FuncDef(FunctionDef),
    PrefixExp(Box<PrefixExpr>),
    // Table(TableConstructor),
    Array(ArrayLiteral),
    Struct(StructLiteral),
    BinExp(BinExp),
    UnExp(UnExp),
}


/// prefixexp '[' exp ']'
#[derive(Debug, PartialEq)]
pub struct IndexExpr {
    pub expr: Box<PrefixExpr>,
    pub arg: Expr,
}

/// exp binop exp
#[derive(Debug, PartialEq)]
pub struct BinExp {
    pub op: BinOp,
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

/// unop exp
#[derive(Debug, PartialEq)]
pub struct UnExp {
    pub op: Unop,
    pub exp: Box<Expr>,
}

/// prefixexp ::= var | functioncall | '(' exp ')'
#[derive(Debug, PartialEq)]
pub enum PrefixExpr {
    Var(Var),
    FunctionCall(FunctionCall),
    Expr(Expr),
}

/// prefixexp '.' Name
#[derive(Debug, PartialEq)]
pub struct PropertyAccess {
    pub expr: Box<PrefixExpr>,
    pub name: Ident,
}
