use crate::common::token::{PunctTyp, Token, DelimTyp, TokenType, LiteralTyp, TokenData};


#[derive(Debug, Clone)]
pub struct Tokeniser<'a> {
    code: &'a String,
    tokens: Vec<Token>,
    f_name: String
}


impl<'a> Tokeniser<'a> {
    pub fn new(code: &'a String, f_name: String) -> Self {
        Self {
            code,
            tokens: Vec::new(),
            f_name
        }
    }

    pub fn parse(&mut self) -> &mut Self {
        let mut code_iter = self.code.chars().into_iter();
        let mut code_iter = crate::common::LocationalIterator::new(&mut code_iter, self.f_name.clone());

        loop {
            let Some(c) = code_iter.peek() else {
                break self;
            };
            let c = c.clone();
            match c {
                'a'..='z' |
                'A'..='Z' | '_' => {
                    let mut buf: String = String::new();

                    loop {
                        if let Some(c) = code_iter.peek() {
                            match c {
                                '0'..='9' |
                                'a'..='z' |
                                'A'..='Z' |
                                '_' => {
                                    buf.push(*c);
                                    let _ = code_iter.next();
                                }
                                _ => {
                                    break;
                                }
                            }
                        } else {
                            break;
                        }
                    }
                    

                    if let Ok(kw) = buf.clone().try_into() {
                        self.tokens.push(Token {
                            loc: code_iter.loc(),
                            typ: TokenType::Keyword,
                            data: TokenData{ kw_typ: Some(kw), val: buf, lit_typ: None, punct_typ: None, delim_typ: None }
                        })
                    } else {
                        self.tokens.push(Token {
                            loc: code_iter.loc(),
                            typ: TokenType::Ident,
                            data: TokenData{ kw_typ: None, val: buf, lit_typ: None, punct_typ: None, delim_typ: None }
                        })
                    }
                }
                '-' | 
                '0'..='9' => {
                    let mut buf: String = String::new();

                    loop {
                        if let Some(c) = code_iter.peek() {
                            match c {
                                '0'..='9' |
                                'a'..='f' |
                                'A'..='F' |
                                'o' | 'x' | '.' => {
                                    buf.push(*c);
                                    let _ = code_iter.next();
                                }
                                _ => {
                                    break;
                                }
                            }
                        } else {
                            break;
                        }
                    }

                    if let Ok(r) = parse_int::parse::<u64>(buf.as_str()) {
                        self.tokens.push(Token {
                            loc: code_iter.loc(),
                            typ: TokenType::Literal,
                            data: TokenData {
                                val: buf,
                                lit_typ: Some(LiteralTyp::UInt { val: r }),
                                kw_typ: None,
                                punct_typ: None,
                                delim_typ: None,
                            }
                        })
                    } else
                    if let Ok(r) = parse_int::parse::<i64>(buf.as_str()) {
                        self.tokens.push(Token {
                            loc: code_iter.loc(),
                            typ: TokenType::Literal,
                            data: TokenData {
                                val: buf,
                                lit_typ: Some(LiteralTyp::Int { val: r }),
                                kw_typ: None,
                                punct_typ: None,
                                delim_typ: None,
                            }
                        })
                    } else
                    if let Ok(r) = parse_int::parse::<f64>(buf.as_str()) {
                        self.tokens.push(Token {
                            loc: code_iter.loc(),
                            typ: TokenType::Literal,
                            data: TokenData {
                                val: buf,
                                lit_typ: Some(LiteralTyp::Float { val: r }),
                                kw_typ: None,
                                punct_typ: None,
                                delim_typ: None,
                            }
                        })
                    }
                }
                
                '"' => {
                    let mut buf = String::new();
                    let mut prev = '\0';
                    let _ = code_iter.next();
                    loop {
                        if let Some(c) = code_iter.peek() {
                            if prev != '\\' && *c == '"'{
                                let _ = code_iter.next();
                                break;
                            } else
                            if prev == '\\' && *c == '\\' {
                                buf.push('\\');
                            } else {
                                buf.push(*c);
                            }
                            prev = *c;
                            let _ = code_iter.next();
                        }
                    }

                    self.tokens.push(Token{
                        loc: code_iter.loc().clone(),
                        typ: TokenType::Literal,
                        data: TokenData { 
                            val: buf.clone(),
                            lit_typ: Some(LiteralTyp::String {val: buf}),
                            kw_typ: None,
                            punct_typ: None,
                            delim_typ: None, 
                        },
                    })
                }
                '\'' => {
                    let mut buf = String::new();
                    let mut prev = '\0';
                    let _ = code_iter.next();
                    loop {
                        if let Some(c) = code_iter.peek() {
                            if prev != '\\' && *c == '\''{
                                let _ = code_iter.next();
                                break;
                            } else
                            if prev == '\\' && *c == '\\' {
                                buf.push('\\');
                            } else {
                                buf.push(*c);
                            }
                            prev = *c;
                            let _ = code_iter.next();
                        }
                    }

                    if buf.len() > 1 {
                        error!("Chars can only have 1 character");
                    }
                    self.tokens.push(Token{
                        loc: code_iter.loc().clone(),
                        typ: TokenType::Literal,
                        data: TokenData {
                            val: buf.clone(),
                            lit_typ: Some(LiteralTyp::Char {
                                val: buf.chars().nth(0).unwrap()
                            }),
                            kw_typ: None,
                            punct_typ: None,
                            delim_typ: None,
                        },
                    })
                }
                ' ' | '\n' | '\t' | '\r' => {
                    let _ = code_iter.next();
                },
                c => {
                    if let Some(typ) = PunctTyp::from_iter(&mut code_iter) {
                        self.tokens.push(Token{
                            loc: code_iter.loc().clone(),
                            typ: TokenType::Punct,
                            data: TokenData {
                                val: String::from(c),
                                kw_typ: None,
                                lit_typ: None,
                                punct_typ: Some(typ),
                                delim_typ: None
                            },
                        }
                    )
                    }
                    if let Some(typ) = DelimTyp::from_iter(&mut code_iter) {
                        self.tokens.push(Token{
                            loc: code_iter.loc().clone(),
                            typ: TokenType::Delim,
                            data: TokenData {
                                val: String::from(c),
                                kw_typ: None,
                                lit_typ: None,
                                punct_typ: None,
                                delim_typ: Some(typ)
                            },
                        })
                    }
                }
            }
        }
        
    }

    pub fn tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }
}