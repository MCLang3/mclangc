#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum LiteralTyp {
    String {val: String},
    Char {val: char},
    Int {val: i64},
    UInt {val: u64},
    Float {val: f64},
}

impl ToString for LiteralTyp {
    fn to_string(&self) -> String {
        match self {
            LiteralTyp::String { val } => format!("String({:?})", val),
            LiteralTyp::Char { val } => format!("Char('{}')", val),
            LiteralTyp::Int { val } => format!("Int({})", val),
            LiteralTyp::UInt { val } => format!("UInt({})", val),
            LiteralTyp::Float { val } => format!("Float({})", val),
        }
    }
}