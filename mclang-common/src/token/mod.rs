
use crate::Loc;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum TokenType {
    Ident {val: String},
    // Keyword
    Let, If, Else, Const, Fn, Struct, Type,
    Include, Pub, Enum, Loop, While, For, 
    Return, Self_, True, False, Break, Goto,

    // Delim
    CurlyOpen,   // {
    CurlyClose,  // }
    SquareOpen,  // [
    SquareClose, // ]
    ParenOpen,   // (
    ParenClose,   // )

    // Literals
    String {val: String},
    Char {val: char},
    Int {val: i64},
    UInt {val: u64},
    Float {val: f64},

    // Punct
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



#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub loc: Loc,
    pub typ: TokenType,
    pub lexem: String,
}




impl ToString for TokenType {
    fn to_string(&self) -> String {
        match self {
            TokenType::CurlyOpen    => String::from("{"),
            TokenType::CurlyClose   => String::from("}"),
            TokenType::SquareOpen   => String::from("["),
            TokenType::SquareClose  => String::from("]"),
            TokenType::ParenOpen    => String::from("("),
            TokenType::ParenClose   => String::from(")"),
            TokenType::Plus         => String::from("+"),
            TokenType::Minus        => String::from("-"),
            TokenType::Star         => String::from("*"),
            TokenType::Slash        => String::from("/"),
            TokenType::Percent      => String::from("%"),
            TokenType::Caret        => String::from("^"),
            TokenType::Not          => String::from("!"),
            TokenType::And          => String::from("&"),
            TokenType::Or           => String::from("|"),
            TokenType::AndAnd       => String::from("&&"),
            TokenType::OrOr         => String::from("||"),
            TokenType::Shl          => String::from("<<"),
            TokenType::Shr          => String::from(">>"),
            TokenType::PlusEq       => String::from("+="),
            TokenType::MinusEq      => String::from("-="),
            TokenType::StarEq       => String::from("*="),
            TokenType::SlashEq      => String::from("/="),
            TokenType::PercentEq    => String::from("%="),
            TokenType::CaretEq      => String::from("^="),
            TokenType::AndEq        => String::from("&="),
            TokenType::OrEq         => String::from("|="),
            TokenType::ShlEq        => String::from("<<="),
            TokenType::ShrEq        => String::from(">>="),
            TokenType::Eq           => String::from("="),
            TokenType::EqEq         => String::from("=="),
            TokenType::Ne           => String::from("!="),
            TokenType::Gt           => String::from(">"),
            TokenType::Lt           => String::from("<"),
            TokenType::Ge           => String::from(">="),
            TokenType::Le           => String::from("<="),
            TokenType::At           => String::from("@"),
            TokenType::Underscore   => String::from("_"),
            TokenType::Dot          => String::from("."),
            TokenType::DotDot       => String::from(".."),
            TokenType::DotDotDot    => String::from("..."),
            TokenType::DotDotEq     => String::from("..="),
            TokenType::Comma        => String::from(","),
            TokenType::Semi         => String::from(";"),
            TokenType::Colon        => String::from(":"),
            TokenType::PathSep      => String::from("::"),
            TokenType::RArrow       => String::from("->"),
            TokenType::FatArrow     => String::from("=>"),
            TokenType::Pound        => String::from("#"),
            TokenType::Dollar       => String::from("$"),
            TokenType::Question     => String::from("?"),
            TokenType::Tilde        => String::from("~"),
            TokenType::Let          => String::from("let"),
            TokenType::If           => String::from("if"),
            TokenType::Else         => String::from("else"),
            TokenType::Const        => String::from("const"),
            TokenType::Fn           => String::from("fn"),
            TokenType::Struct       => String::from("struct"),
            TokenType::Type         => String::from("type"),
            TokenType::Include      => String::from("include"),
            TokenType::Pub          => String::from("pub"),
            TokenType::Enum         => String::from("enum"),
            TokenType::Loop         => String::from("loop"),
            TokenType::While        => String::from("while"),
            TokenType::For          => String::from("for"),
            TokenType::Return       => String::from("return"),
            TokenType::Self_        => String::from("Self"),
            TokenType::True         => String::from("true"),
            TokenType::False        => String::from("false"),
            TokenType::Break        => String::from("break"),
            TokenType::Goto        => String::from("goto"),
            TokenType::String { val } => format!("String({:?})", val),
            TokenType::Char   { val } => format!("Char('{}')", val),
            TokenType::Int    { val } => format!("Int({})", val),
            TokenType::UInt   { val } => format!("UInt({})", val),
            TokenType::Float  { val } => format!("Float({})", val),
            TokenType::Ident  { val } => format!("Ident({})", val),
        }.to_string()
    }
}


pub fn try_into_kw(s: &str) -> Option<TokenType> {
    match s {
        "let"     => Some(TokenType::Let),
        "if"      => Some(TokenType::If),
        "else"    => Some(TokenType::Else),
        "const"   => Some(TokenType::Const),
        "fn"      => Some(TokenType::Fn),
        "struct"  => Some(TokenType::Struct),
        "type"    => Some(TokenType::Type),
        "include" => Some(TokenType::Include),
        "pub"     => Some(TokenType::Pub),
        "enum"    => Some(TokenType::Enum),
        "loop"    => Some(TokenType::Loop),
        "while"   => Some(TokenType::While),
        "for"     => Some(TokenType::For ),
        "return"  => Some(TokenType::Return),
        "Self"    => Some(TokenType::Self_),
        "true"    => Some(TokenType::True),
        "false"   => Some(TokenType::False),
        "break"   => Some(TokenType::Break),
        "goto"    => Some(TokenType::Goto),
        _ => None
    }
}

pub fn try_into_delim(i: &mut crate::LocationalIterator) -> Option<TokenType> {
    match i.peek() {
        Some(c) => {
            match c {
                '{' => {
                    let _ = i.next();
                    Some(TokenType::CurlyOpen)
                }
                '}' => {
                    let _ = i.next();
                    Some(TokenType::CurlyClose)
                }
                '[' => {
                    let _ = i.next();
                    Some(TokenType::SquareOpen)
                }
                ']' => {
                    let _ = i.next();
                    Some(TokenType::SquareClose)
                }
                '(' => {
                    let _ = i.next();
                    Some(TokenType::ParenOpen)
                }
                ')' => {
                    let _ = i.next();
                    Some(TokenType::ParenClose)
                }
                _ => None
            }
        }
        _ => None
    }
}

pub fn try_into_punct(s: &mut crate::LocationalIterator) -> Option<TokenType> {
    match s.peek() {
        Some(c) => {
            match c {
                '@' => {
                    let _ = s.next();
                    Some(TokenType::At)
                }
                '_' => {
                    let _ = s.next();
                    Some(TokenType::Underscore)
                }
                ',' => {
                    let _ = s.next();
                    Some(TokenType::Comma)
                }
                ';' => {
                    let _ = s.next();
                    Some(TokenType::Semi)
                }
                '#' => {
                    let _ = s.next();
                    Some(TokenType::Pound)
                }
                '$' => {
                    let _ = s.next();
                    Some(TokenType::Dollar)
                }
                '?' => {
                    let _ = s.next();
                    Some(TokenType::Question)
                }
                '~' => {
                    let _ = s.next();
                    Some(TokenType::Tilde)
                }
                '+' => {
                    let _ = s.next();
                    match s.peek() {
                        Some('=') => {
                            let _ = s.next();
                            Some(TokenType::PlusEq)
                        }
                        _ => Some(TokenType::Plus)
                    }
                }
                '-' => {
                    let _ = s.next();
                    match s.peek() {
                        Some('=') => {
                            let _ = s.next();
                            Some(TokenType::MinusEq)
                        }
                        Some('>') => {
                            let _ = s.next();
                            Some(TokenType::RArrow)
                        }
                        _ => Some(TokenType::Minus)
                    }
                }
                '*' => {
                    let _ = s.next();
                    match s.peek() {
                        Some('=') => {
                            let _ = s.next();
                            Some(TokenType::StarEq)
                        }
                        _ => Some(TokenType::Star)
                    }
                }
                '/' => {
                    let _ = s.next();
                    match s.peek() {
                        Some('=') => {
                            let _ = s.next();
                            Some(TokenType::SlashEq)
                        }
                        _ => Some(TokenType::Slash)
                    }
                }
                '%' => {
                    let _ = s.next();
                    match s.peek() {
                        Some('=') => {
                            let _ = s.next();
                            Some(TokenType::PercentEq)
                        }
                        _ => Some(TokenType::Percent)
                    }
                }
                '^' => {
                    match s.peek() {
                        Some('=') => {
                            let _ = s.next();
                            Some(TokenType::CaretEq)
                        }
                        _ => Some(TokenType::Caret)
                    }
                }
                '!' => {
                    let _ = s.next();
                    match s.peek() {
                        Some('=') => {
                            let _ = s.next();
                            Some(TokenType::Ne)
                        }
                        _ => Some(TokenType::Not)
                    }
                }
                '&' => {
                    let _ = s.next();
                    match s.peek() {
                        Some('&') => {
                            let _ = s.next();
                            Some(TokenType::AndAnd)
                        }
                        Some('=') => {
                            let _ = s.next();
                            Some(TokenType::AndEq)
                        }
                        _ => Some(TokenType::And)
                    }
                }
                '|' => {
                    let _ = s.next();
                    match s.peek() {
                        Some('|') => {
                            let _ = s.next();
                            Some(TokenType::OrOr)
                        }
                        Some('=') => {
                            let _ = s.next();
                            Some(TokenType::OrEq)
                        }
                        _ => Some(TokenType::Or)
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
                                    Some(TokenType::ShlEq)
                                }
                                _ => Some(TokenType::Shl)
                            }
                        },
                        Some('=') => {
                            let _ = s.next();
                            Some(TokenType::Le)
                        },
                        _ => Some(TokenType::Lt)
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
                                    Some(TokenType::ShrEq)
                                }
                                _ => Some(TokenType::Shr)
                            }
                        },
                        Some('=') => {
                            let _ = s.next();
                            Some(TokenType::Ge)
                        },
                        _ => Some(TokenType::Gt)
                    }
                }
                '=' => {
                    let _ = s.next();
                    match s.peek() {
                        Some('=') => {
                            let _ = s.next();
                            Some(TokenType::EqEq)
                        }
                        Some('>') => {
                            let _ = s.next();
                            Some(TokenType::FatArrow)
                        }
                        _ => Some(TokenType::Eq)
                    }
                }
                ':' => {
                    let _ = s.next();
                    match s.peek() {
                        Some(':') => {
                            let _ = s.next();
                            Some(TokenType::PathSep)
                        }
                        _ => Some(TokenType::Colon)
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
                                    Some(TokenType::DotDotDot)
                                }
                                Some('=') => {
                                    let _ = s.next();
                                    Some(TokenType::DotDotEq)
                                }
                                _ => Some(TokenType::DotDot)
                            }
                        }
                        _ => Some(TokenType::Dot)
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