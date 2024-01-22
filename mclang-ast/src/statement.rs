use crate::{Ident, expression::{Expr, self}, function::{FunctionDef, self}};



#[derive(Debug, PartialEq)]
pub enum Stat {
    SemiColon,
    Assignment(Assignment), // varlist '=' explist
    FunctionCall(function::FunctionCall),
    Label(Ident),
    Break,
    Goto(Ident),
    DoBlock(Block),
    WhileBlock(WhileBlock),
    LoopBlock(RepeatBlock),
    IfBlock(Box<IfBlock>),
    ForRange(Box<ForRange>),
    ForIn(ForIn),
    FunctionDef(FunctionDef),
    // LocalFunctionDef(LocalFunctionDef),
    // LocalAssignment(LocalAssignment),
}

/// while exp do block end
#[derive(Debug, PartialEq)]
pub struct WhileBlock {
    pub expr: Expr,
    pub block: Block,
}

/// repeat block until exp
#[derive(Debug, PartialEq)]
pub struct RepeatBlock {
    pub block: Block,
    pub expr: Expr,
}

/// elseif exp then block
#[derive(Debug, PartialEq)]
pub struct ElseIf {
    pub expr: Expr,
    pub block: Block,
}

/// if exp then block {elseif exp then block} [else block] end
#[derive(Debug, PartialEq)]
pub struct IfBlock {
    pub expr: Expr,
    pub block: Block,
    pub elseif: Vec<ElseIf>,
    pub else_blk: Option<Block>,
}

/// block ::= {stat} [retstat]
#[derive(Debug, PartialEq)]
pub struct Block {
    pub stats: Vec<Stat>,
    pub retstat: Option<Vec<Expr>>,
}

/// for Name '=' exp ',' exp [',' exp] do block end
#[derive(Debug, PartialEq)]
pub struct ForRange {
    pub name: Ident,
    pub exprs: (Expr, Expr, Option<Expr>),
    pub block: Block,
}

/// for namelist in explist do block end
#[derive(Debug, PartialEq)]
pub struct ForIn {
    pub namelist: Vec<Ident>,
    pub exprlist: Vec<Expr>,
    pub block: Block,
}

/// varlist '=' explist
#[derive(Debug, PartialEq)]
pub struct Assignment {
    pub varlist: Vec<Var>,
    pub exprlist: Vec<Expr>,
}

/// var ::=  Name | prefixexp '[' exp ']' | prefixexp '.' Name
#[derive(Debug, PartialEq)]
pub enum Var {
    Name(Ident),
    IndexExpr(expression::IndexExpr),
    PropertyAccess(expression::PropertyAccess),
}
