
mod keyword;
mod delimeter;
mod punctuation;
mod literal;
pub use keyword::*;
pub use delimeter::*;
pub use punctuation::*;
pub use literal::*;

use crate::common::Loc;
use anyhow::Result;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum TokenType {
    Ident,
    Keyword,
    Literal,
    Punct,
    Delim,
}


#[derive(Debug, Clone, Default)]
pub struct TokenData {
    pub val: String,
    pub kw_typ: Option<KeywordType>,
    pub lit_typ: Option<LiteralTyp>,
    pub punct_typ: Option<PunctTyp>,
    pub delim_typ: Option<DelimTyp>
}


#[derive(Debug, Clone)]
pub struct Token {
    pub loc: Loc,
    pub typ: TokenType,
    pub data: TokenData,
}



// impl ToString for TokenType {
//     fn to_string(&self) -> String {
//         match &self {
//             TokenType::Ident { val } => val.clone(),
//             TokenType::Keyword { typ } => typ.to_string(),
//             TokenType::Literal { val: _, typ } => typ.to_string(),
//             TokenType::Punct { typ } => typ.to_string(),
//             TokenType::Delim { typ } => typ.to_string(),
//         }
//     }
// }

// impl Token {
//     pub fn eq_parser(self, r: Token) -> bool {
//         match self {
//             s if s.typ == r.typ => {
//                 true
//             },
//             s => {
                
//             }
//         }
//     }
// }

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match &self.typ {
            TokenType::Ident  => format!("Identifier(\"{}\")", self.data.val.clone()),
            TokenType::Keyword  => self.data.kw_typ.unwrap().to_string(),
            TokenType::Literal => self.data.clone().lit_typ.unwrap().to_string(),
            TokenType::Punct => self.data.punct_typ.unwrap().to_string(),
            TokenType::Delim => self.data.delim_typ.unwrap().to_string(),
        };
        write!(f, "{}", s)?;
        Ok(())
    }
}
impl TokenType {
    pub fn fmt(&self, data: &Option<TokenData>) -> String  {
        match data {
            Some(data) => {
                match &self {
                    TokenType::Ident  => format!("Identifier(\"{}\")", data.val.clone()),
                    TokenType::Keyword  => data.kw_typ.unwrap().to_string(),
                    TokenType::Literal => data.lit_typ.clone().unwrap().to_string(),
                    TokenType::Punct => data.punct_typ.unwrap().to_string(),
                    TokenType::Delim => data.delim_typ.unwrap().to_string(),
                }
            }

            None => {
                match &self {
                    TokenType::Ident  => String::from("Ident"),
                    TokenType::Keyword  => String::from("Keyword"),
                    TokenType::Literal =>  String::from("Literal"),
                    TokenType::Punct =>  String::from("Punct"),
                    TokenType::Delim =>  String::from("Delim"),
                }
            }
        }
    }
}

// impl ToString for Token {
//     fn to_string(&self) -> String {
//         self.typ.to_string()
//     }
// }

