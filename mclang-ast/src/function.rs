use crate::{statement::Block, Ident, expression};


/// function funcname funcbody
#[derive(Debug, PartialEq)]
pub struct FunctionDef {
    pub name: FuncName,
    pub body: FuncBody,
}

/// funcname ::= Name {'.' Name} [':' Name]
#[derive(Debug, PartialEq)]
pub struct FuncName {
    pub path: Vec<Ident>,
    pub method: Option<Ident>,
}

/// functioncall ::=  prefixexp args | prefixexp ':' Name args
#[derive(Debug, PartialEq)]
pub struct FunctionCall {
    pub expr: Box<expression::PrefixExpr>,
    pub args: Args,
}

/// args ::=  '(' [explist] ')' | tableconstructor | LiteralString
#[derive(Debug, PartialEq)]
pub enum Args {
    ExprList(Vec<expression::Expr>),
}

/// funcbody ::= '(' [parlist] ')' block end
#[derive(Debug, PartialEq)]
pub struct FuncBody {
    pub params: Params,
    pub body: Block,
}

/// parlist ::= namelist [',' '...'] | '...'
#[derive(Debug, PartialEq)]
pub struct Params {
    pub names: Vec<Ident>,
    pub variadic: bool,
}

