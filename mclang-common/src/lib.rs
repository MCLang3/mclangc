
use token::{TokenType, Token};
pub mod token;
pub mod logger;
#[macro_use]
pub mod macros;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Loc{
    pub file: String,
    pub line: usize,
    pub col:  usize
}

// For mclang_parser
impl From<Option<&Token>> for Loc {
    fn from(val: Option<&Token>) -> Self {
        if let Some(t) = val {
            t.loc.clone()
        } else {
            Loc::default()
        }
    }
}

impl Loc {
    pub fn new(file: String) -> Self {
        Self {
            file,
            line: 1,
            col: 0,
        }
    }
    #[allow(dead_code)]
    pub fn human(&self) -> String {
        format!("{}:{}:{}", self.file, self.line, self.col)
    }
}

pub struct LocationalIterator{
    // iter: Peekable<&'a mut dyn Iterator<char=char>>,
    chars: Vec<char>,
    loc: Loc
}

// impl<char> Clone for LocationalIterator<char> {
//     fn clone(&self) -> Self {

//         // This is nasty, and extremely slow
//         // let chrs = String::from_utf8(self.iter.map(|c| c as u8 ).collect::<Vec<u8>>().clone()).unwrap().chars();


//         // Probably safe as were copying to the same type
//         unsafe {
//             Self { iter: std::mem::transmute_copy(&self.iter), loc: self.loc.clone() }
//         }
//     }
// }

// impl<char> Debug for LocationalIterator<char> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("LocationalIterator").field("iter", &"[excluded]").field("loc", &self.loc).finish()
//     }
// }

impl<'a> LocationalIterator {
    pub fn new(arr: Vec<char>, f_name: String) -> Self{
        Self {
            chars: arr,
            loc: Loc::new(f_name),
        }
    }

    pub fn peek(&mut self) -> Option<&char> {
        self.chars.last()
    }

    #[allow(dead_code)]
    pub fn peek_mut(&mut self) -> Option<&mut char> {
        self.chars.last_mut()
    }

    pub fn next(&mut self) -> Option<char> {
        let a = self.chars.pop();
        if let Some(a) = a {
            match a.into() {
                '\n' => {
                    self.loc.line += 1;
                    self.loc.col = 1;
                }
                _ => {
                    self.loc.col += 1;
                }
            }
        }
        a.clone()
    }
    pub fn loc(&mut self) -> Loc {
        self.loc.clone()
    }

}