use std::iter::Peekable;
pub mod token;
pub mod logger;
#[macro_use]
pub mod macros;

#[derive(Debug, Clone, Default)]
pub struct Loc{
    pub file: String,
    pub line: usize,
    pub col:  usize
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

pub struct LocationalIterator<'a>{
    iter: Peekable<&'a mut dyn Iterator<Item=char>>,
    loc: Loc
}

impl<'a> LocationalIterator<'a> {
    pub fn new(iter: &'a mut dyn Iterator<Item=char>, f_name: String) -> Self{
        Self {
            iter: iter.peekable(),
            loc: Loc::new(f_name),
        }
    }

    pub fn peek(&mut self) -> Option<&char> {
        let a = self.iter.peek();
        a
    }

    #[allow(dead_code)]
    pub fn peek_mut(&mut self) -> Option<&mut char> {
        let a = self.iter.peek_mut();
        a
    }

    pub fn next(&mut self) -> Option<char> {
        let a = self.iter.next();
        if let Some(a) = a {
            match a {
                '\n' => {
                    self.loc.line += 1;
                    self.loc.col = 1;
                }
                _ => {
                    self.loc.col += 1;
                }
            }
        };
        a
    }
    pub fn loc(&mut self) -> Loc {
        self.loc.clone()
    }

}
