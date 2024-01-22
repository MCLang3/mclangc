
/// unop ::= '-' | not | '#' | '~'
#[derive(Debug, PartialEq)]
pub enum Unop {
    Minus,
    Not,
    Len,
    BitNot,
}

/// binop ::=  '+' | '-' | '*' | '/' | '//' | '^' | '%' |
///         '&' | '~' | '|' | '>>' | '<<' | '..' |
///         '<' | '<=' | '>' | '>=' | '==' | '~=' |
///         and | or
#[derive(Debug, PartialEq)]
pub enum BinOp {
    Plus,
    Minus,
    Mul,
    Div,
    IntDiv,
    Pow,
    Mod,
    BitAnd,
    BitXor,
    BitOr,
    BitShr,
    BitShl,
    Concat,
    LT,
    LTE,
    GT,
    GTE,
    EQ,
    NEQ,
    And,
    Or,
}
