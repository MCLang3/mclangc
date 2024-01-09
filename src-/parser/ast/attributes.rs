use super::ASTNode;

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum ASTAttr {
    Text(String),
    Num(u64),
    Cfg(Box<ASTAttr>),
    Not(Box<ASTAttr>),
    Any(Vec<ASTAttr>),
    All(Vec<ASTAttr>),
    Bin(Box<ASTNode>)
}