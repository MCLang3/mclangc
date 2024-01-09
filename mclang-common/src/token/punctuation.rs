
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Default)]
pub enum PunctTyp {
    #[default]
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

impl PunctTyp {
    pub fn from_iter(s: &mut crate::LocationalIterator) -> Option<Self> {
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

impl Into<&str> for PunctTyp {
    fn into(self) -> &'static str {
        match self {
            Self::Plus       => "+",
            Self::Minus      => "-",
            Self::Star       => "*",
            Self::Slash      => "/",
            Self::Percent    => "%",
            Self::Caret      => "^",
            Self::Not        => "!",
            Self::And        => "&",
            Self::Or         => "|",
            Self::AndAnd     => "&&",
            Self::OrOr       => "||",
            Self::Shl        => "<<",
            Self::Shr        => ">>",
            Self::PlusEq     => "+=",
            Self::MinusEq    => "-=",
            Self::StarEq     => "*=",
            Self::SlashEq    => "/=",
            Self::PercentEq  => "%=",
            Self::CaretEq    => "^=",
            Self::AndEq      => "&=",
            Self::OrEq       => "|=",
            Self::ShlEq      => "<<=",
            Self::ShrEq      => ">>=",
            Self::Eq         => "=",
            Self::EqEq       => "==",
            Self::Ne         => "!=",
            Self::Gt         => ">",
            Self::Lt         => "<",
            Self::Ge         => ">=",
            Self::Le         => "<=",
            Self::At         => "@",
            Self::Underscore => "_",
            Self::Dot        => ".",
            Self::DotDot     => "..",
            Self::DotDotDot  => "...",
            Self::DotDotEq   => "..=",
            Self::Comma      => ",",
            Self::Semi       => ";",
            Self::Colon      => ":",
            Self::PathSep    => "::",
            Self::RArrow     => "->",
            Self::FatArrow   => "=>",
            Self::Pound      => "#",
            Self::Dollar     => "$",
            Self::Question   => "?",
            Self::Tilde      => "~",
        }
    }
}

impl Into<String> for PunctTyp {
    fn into(self) -> String {
        let a: &str = self.into();
        a.to_string()
    }
}

impl ToString for PunctTyp {
    fn to_string(&self) -> String {
        (*self).into()
    }
}