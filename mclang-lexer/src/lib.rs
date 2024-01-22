use mclang_common::{token::{Token, TokenType, try_into_kw, try_into_punct, try_into_delim}, error};


#[derive(Debug, Clone)]
pub struct Tokeniser<'a> {
    code: &'a String,
    tokens: Vec<Token>,
    f_name: String
}


macro_rules! push_token {
    ($self:expr, $ci:expr, $lexem:expr, $tt:expr) => {
        $self.tokens.push(Token {
            loc: $ci.loc(),
            typ: $tt,
            lexem: $lexem
        });
    };
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
        let mut code_iter = mclang_common::LocationalIterator::new(self.code.chars().collect(), self.f_name.clone());

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
                    

                    if let Some(kw) = try_into_kw(&buf) {
                        push_token!(self, code_iter, buf, kw);
                    } else {
                        push_token!(self, code_iter, buf, TokenType::Ident { val: buf.clone() });
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
                        push_token!(self, code_iter, buf, TokenType::UInt { val: r });
                    } else
                    if let Ok(r) = parse_int::parse::<i64>(buf.as_str()) {
                        push_token!(self, code_iter, buf, TokenType::Int {val: r});
                    } else
                    if let Ok(r) = parse_int::parse::<f64>(buf.as_str()) {
                        push_token!(self, code_iter, buf, TokenType::Float { val: r });
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

                    push_token!(self, code_iter, buf, TokenType::String { val: buf.clone() });
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
                    push_token!(self, code_iter, buf, TokenType::Char { val: buf.chars().nth(0).unwrap() });
                }
                ' ' | '\n' | '\t' | '\r' => {
                    let _ = code_iter.next();
                },
                _ => {
                    if let Some(typ) = try_into_punct(&mut code_iter) {
                        push_token!(self, code_iter, typ.to_string(), typ.clone());
                    }
                    if let Some(typ) = try_into_delim(&mut code_iter) {
                        push_token!(self, code_iter, typ.to_string(), typ.clone());
                    }
                }
            }
        }
        
    }

    pub fn tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }
}


