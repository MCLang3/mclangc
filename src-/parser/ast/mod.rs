pub mod attributes;
pub mod common;

use std::collections::HashMap;
use std::default;

use crate::common::Loc;
use crate::common::token::PunctTyp;
use crate::parser::ast::common::ASTCommon;

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum ASTLiteral {
    String(String),
    Char(char),
    Int(u64),
    UInt(i64),
    Float(f64),
}
impl Default for ASTLiteral {
    fn default() -> Self {
        Self::String(String::default())
    }
}

#[derive(Debug, Clone, Default)]
pub struct Program {
    pub com: ASTCommon,
    pub body: Vec<ASTNode>,
}

#[derive(Debug, Clone, Default)]
pub struct Ident {
    pub com: ASTCommon,
    pub name: String,
}

#[derive(Debug, Clone, Default)]
pub struct Function {
    pub com: ASTCommon,
    pub name: Box<ASTNode>,
    pub args: HashMap<ASTNode, ASTNode>,
    pub body: Box<ASTNode>,
    pub public: bool,
}

#[derive(Debug, Clone, Default)]
pub struct Variable {
    pub com: ASTCommon,
    pub name: Box<ASTNode>,
    pub mutable: bool,
    pub public: bool,
    pub assignment: Box<ASTNode>,
}

#[derive(Debug, Clone, Default)]
pub struct If {
    pub com: ASTCommon,
    pub condition: Box<ASTNode>,
    pub body: Box<ASTNode>,
    pub r#else: Vec<ASTNode>,
}

#[derive(Debug, Clone, Default)]
pub struct While {
    pub com: ASTCommon,
    pub condition: Box<ASTNode>,
    pub body: Vec<ASTNode>,
}

#[derive(Debug, Clone, Default)]
pub struct For {
    pub com: ASTCommon,
    pub init: Box<ASTNode>,
    pub condition: Box<ASTNode>,
    pub on_loop: Box<ASTNode>,
    pub body: Vec<ASTNode>,
}

#[derive(Debug, Clone, Default)]
pub struct StructDef {
    pub com: ASTCommon,
    pub name: Box<ASTNode>,
    pub fields: HashMap<ASTNode, ASTNode>,
}

#[derive(Debug, Clone, Default)]
pub struct Statement {
    pub com: ASTCommon,
    pub right: Box<ASTNode>,
    pub left: Box<ASTNode>,
    pub typ: PunctTyp,
}

#[derive(Debug, Clone, Default)]
pub struct Expression {
    pub com: ASTCommon,
    pub right: Box<ASTNode>,
    pub left: Box<ASTNode>,
    pub typ: PunctTyp,
}

#[derive(Debug, Clone, Default)]
pub struct FunctionCall {
    pub com: ASTCommon,
    pub name: Box<ASTNode>,
    pub args: Vec<ASTNode>,
}

#[derive(Debug, Clone, Default)]
pub struct Block {
    pub com: ASTCommon,
    pub body: Vec<ASTNode>,
}

#[derive(Debug, Clone, Default)]
pub struct Module {
    pub com: ASTCommon,
    pub name: Box<ASTNode>,
    pub body: Option<Box<ASTNode>>,
}

#[derive(Debug, Clone, Default)]
pub struct Import {
    pub com: ASTCommon,
    pub path: String,
    pub r#as: Option<Box<ASTNode>>,
}

#[derive(Debug, Clone, Default)]
pub struct Literal {
    pub com: ASTCommon,
    pub val: ASTLiteral,
}

#[derive(Debug, Clone, Default)]
pub struct StructInit {
    pub com: ASTCommon,
    pub fields: HashMap<ASTNode, ASTNode>,
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum ASTNode {
    Program(Program),
    Ident(Ident),
    Function(Function),
    Variable(Variable),
    If(If),
    While(While),
    For(For),
    StructDef(StructDef),
    Statement(Statement),
    Expression(Expression),
    FunctionCall(FunctionCall),
    Block(Block),
    Module(Module),
    Import(Import),
    Literal(Literal),
    StructInit(StructInit),
}

impl Default for ASTNode {
    fn default() -> Self {
        Self::Program(Program{ com: ASTCommon::default(), body: Vec::default()})
    }
}

impl ASTNode {
    pub fn new(file: String) -> Self {
        Self::Program(Program{ com: ASTCommon { loc: Loc{
            file,
            line: 0,
            col: 0,
        }, attr: HashMap::new() }, body: Vec::new() })
    }
}