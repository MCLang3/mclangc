
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub enum DelimTyp {
    CurlyOpen,   // {
    CurlyClose,  // }
    SquareOpen,  // [
    SquareClose, // ]
    ParenOpen,   // (
    ParenClose   // )
}

impl DelimTyp {
    pub fn from_iter(i: &mut crate::common::LocationalIterator) -> Option<Self> {
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

impl ToString for DelimTyp {
    fn to_string(&self) -> String {
        match self {
            DelimTyp::CurlyOpen => "{",
            DelimTyp::CurlyClose => "}",
            DelimTyp::SquareOpen => "[",
            DelimTyp::SquareClose => "]",
            DelimTyp::ParenOpen => "(",
            DelimTyp::ParenClose => ")",
        }.to_string()
    }
}