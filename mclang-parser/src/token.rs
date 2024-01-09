
use crate::common::{Loc, LocationalIterator};

#[derive(Debug, Clone)]
pub enum TokenType {
    Ident   {
        val: String,
        typ: Option<KeywordType>
    },
    Literal {
        val: String,
        typ: LiteralTyp
    },
    Punct  {
        typ: PunctTyp
    },
    Delim  {
        typ: DelimTyp
    }
}

#[derive(Debug, Clone)]
pub enum LiteralTyp {
    String,
    Char,
    Int {val: i64},
    UInt {val: u64},
    Float {val: f64},
}

#[derive(Debug, Clone, Copy)]
pub enum KeywordType {
    Let, If, Else, Const, Fn, Struct, Type,
    Include, Pub, Enum, Loop, While, For, 
    Return, Self_, True, False
}

#[derive(Debug, Clone, Copy)]
pub enum PunctTyp {
    Plus,       // +
    Minus,      // -
    Star,       // *
    Slash,      // /
    Percent,    // %
    Caret,      // ^
    Not,        // !
    And,        // &
    Or,         // |
    AndAnd,     // &&
    OrOr,       // ||
    Shl,        // <<
    Shr,        // >>
    PlusEq,     // +=
    MinusEq,    // -=
    StarEq,     // *=
    SlashEq,    // /=
    PercentEq,  // %=
    CaretEq,    // ^=
    AndEq,      // &=
    OrEq,       // |=
    ShlEq,      // <<=
    ShrEq,      // >>=
    Eq,         // =
    EqEq,       // ==
    Ne,         // !=
    Gt,         // >
    Lt,         // <
    Ge,         // >=
    Le,         // <=
    At,         // @
    Underscore, // _
    Dot,        // .
    DotDot,     // ..
    DotDotDot,  // ...
    DotDotEq,   // ..=
    Comma,      // ,
    Semi,       // ;
    Colon,      // :
    PathSep,    // ::
    RArrow,     // ->
    FatArrow,   // =>
    Pound,      // #
    Dollar,     // $
    Question,   // ?
    Tilde,      // ~
}

#[derive(Debug, Clone, Copy)]
pub enum DelimTyp {
    CurlyOpen,   // {
    CurlyClose,  // }
    SquareOpen,  // [
    SquareClose, // ]
    ParenOpen,   // (
    ParenClose   // )
}

#[derive(Debug, Clone)]
pub struct Token {
    pub loc: Loc,
    pub typ: TokenType
}

impl PunctTyp {
    pub fn from_char(s: &mut LocationalIterator) -> Option<Self> {
        match s.peek() {
            Some(c) => {
                match c {
                    '@' => {
                        let _ = s.next();
                        Some(Self::At)
                    }
                    '_' => {
                        let _ = s.next();
                        Some(Self::Underscore)
                    }
                    ',' => {
                        let _ = s.next();
                        Some(Self::Comma)
                    }
                    ';' => {
                        let _ = s.next();
                        Some(Self::Semi)
                    }
                    '#' => {
                        let _ = s.next();
                        Some(Self::Pound)
                    }
                    '$' => {
                        let _ = s.next();
                        Some(Self::Dollar)
                    }
                    '?' => {
                        let _ = s.next();
                        Some(Self::Question)
                    }
                    '~' => {
                        let _ = s.next();
                        Some(Self::Tilde)
                    }
                    '+' => {
                        let _ = s.next();
                        match s.peek() {
                            Some('=') => {
                                let _ = s.next();
                                Some(Self::PlusEq)
                            }
                            _ => Some(Self::Plus)
                        }
                    }
                    '-' => {
                        let _ = s.next();
                        match s.peek() {
                            Some('=') => {
                                let _ = s.next();
                                Some(Self::MinusEq)
                            }
                            Some('>') => {
                                let _ = s.next();
                                Some(Self::RArrow)
                            }
                            _ => Some(Self::Minus)
                        }
                    }
                    '*' => {
                        let _ = s.next();
                        match s.peek() {
                            Some('=') => {
                                let _ = s.next();
                                Some(Self::StarEq)
                            }
                            _ => Some(Self::Star)
                        }
                    }
                    '/' => {
                        let _ = s.next();
                        match s.peek() {
                            Some('=') => {
                                let _ = s.next();
                                Some(Self::SlashEq)
                            }
                            _ => Some(Self::Slash)
                        }
                    }
                    '%' => {
                        let _ = s.next();
                        match s.peek() {
                            Some('=') => {
                                let _ = s.next();
                                Some(Self::PercentEq)
                            }
                            _ => Some(Self::Percent)
                        }
                    }
                    '^' => {
                        match s.peek() {
                            Some('=') => {
                                let _ = s.next();
                                Some(Self::CaretEq)
                            }
                            _ => Some(Self::Caret)
                        }
                    }
                    '!' => {
                        let _ = s.next();
                        match s.peek() {
                            Some('=') => {
                                let _ = s.next();
                                Some(Self::Ne)
                            }
                            _ => Some(Self::Not)
                        }
                    }
                    '&' => {
                        let _ = s.next();
                        match s.peek() {
                            Some('&') => {
                                let _ = s.next();
                                Some(Self::AndAnd)
                            }
                            Some('=') => {
                                let _ = s.next();
                                Some(Self::AndEq)
                            }
                            _ => Some(Self::And)
                        }
                    }
                    '|' => {
                        let _ = s.next();
                        match s.peek() {
                            Some('|') => {
                                let _ = s.next();
                                Some(Self::OrOr)
                            }
                            Some('=') => {
                                let _ = s.next();
                                Some(Self::OrEq)
                            }
                            _ => Some(Self::Or)
                        }
                    }
                    '<' => {
                        let _ = s.next();
                        match s.peek() {
                            Some('<') => {
                                let _ = s.next();
                                match s.peek() {
                                    Some('=') => {
                                        let _ = s.next();
                                        Some(Self::ShlEq)
                                    }
                                    _ => Some(Self::Shl)
                                }
                            },
                            Some('=') => {
                                let _ = s.next();
                                Some(Self::Le)
                            },
                            _ => Some(Self::Lt)
                        }
                    }
                    '>' => {
                        let _ = s.next();
                        match s.peek() {
                            Some('>') => {
                                let _ = s.next();
                                match s.peek() {
                                    Some('=') => {
                                        let _ = s.next();
                                        Some(Self::ShrEq)
                                    }
                                    _ => Some(Self::Shr)
                                }
                            },
                            Some('=') => {
                                let _ = s.next();
                                Some(Self::Ge)
                            },
                            _ => Some(Self::Gt)
                        }
                    }
                    '=' => {
                        let _ = s.next();
                        match s.peek() {
                            Some('=') => {
                                let _ = s.next();
                                Some(Self::EqEq)
                            }
                            Some('>') => {
                                let _ = s.next();
                                Some(Self::FatArrow)
                            }
                            _ => Some(Self::Eq)
                        }
                    }
                    ':' => {
                        let _ = s.next();
                        match s.peek() {
                            Some(':') => {
                                let _ = s.next();
                                Some(Self::PathSep)
                            }
                            _ => Some(Self::Colon)
                        }
                    },
                    '.' => {
                        let _ = s.next();
                        match s.peek() {
                            Some('.') => {
                                let _ = s.next();
                                match s.peek() {
                                    Some('.') => {
                                        let _ = s.next();
                                        Some(Self::DotDotDot)
                                    }
                                    Some('=') => {
                                        let _ = s.next();
                                        Some(Self::DotDotEq)
                                    }
                                    _ => Some(Self::DotDot)
                                }
                            }
                            _ => Some(Self::Dot)
                        }
                    }

                    _ => None
                }
            }
            None => {
                None
            }
        }
    }
}

impl DelimTyp {
    pub fn from_char(i: &mut LocationalIterator) -> Option<Self> {
        match i.peek() {
            Some(c) => {
                match c {
                    '{' => {
                        let _ = i.next();
                        Some(Self::CurlyOpen)
                    }
                    '}' => {
                        let _ = i.next();
                        Some(Self::CurlyClose)
                    }
                    '[' => {
                        let _ = i.next();
                        Some(Self::SquareOpen)
                    }
                    ']' => {
                        let _ = i.next();
                        Some(Self::SquareClose)
                    }
                    '(' => {
                        let _ = i.next();
                        Some(Self::ParenOpen)
                    }
                    ')' => {
                        let _ = i.next();
                        Some(Self::ParenClose)
                    }
                    _ => None
                }
            }
            _ => None
        }
    }
}

impl KeywordType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "let" => Some(Self::Let),
            "if" => Some(Self::If),
            "else" => Some(Self::Else),
            "const" => Some(Self::Const),
            "fn" => Some(Self::Fn),
            "struct" => Some(Self::Struct),
            "type" => Some(Self::Type),
            "include" => Some(Self::Include),
            "pub" => Some(Self::Pub),
            "enum" => Some(Self::Enum),
            "loop" => Some(Self::Loop),
            "while" => Some(Self::While),
            "for " => Some(Self::For ),
            "return" => Some(Self::Return),
            "Self" => Some(Self::Self_),
            "true" => Some(Self::True),
            "false" => Some(Self::False),
            _ => None
        }
    }
}