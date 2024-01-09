use std::collections::HashMap;
use std::iter::Peekable;
use anyhow::{Result, bail};
use crate::common::token::{Token, TokenType, KeywordType, DelimTyp, TokenData, PunctTyp};

use crate::parser::ast::ASTNode;

use self::ast::common::ASTCommon;


pub mod ast;
#[macro_use]
mod macros;

pub struct Parser<'a> {
    ast: ASTNode,
    tokens: Peekable<std::slice::Iter<'a, Token>>,
    // errors: (usize, usize) // (errors, warns)
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Peekable<std::slice::Iter<'a, Token>>, file: String) -> Self {
        // let mut tokens = tokens.iter().peekable();
        Self {
            ast: ASTNode::new(file),
            tokens: tokens.clone(),
            // errors: (0, 0),
        }
    }

    pub fn start(&mut self) -> Result<&mut Self> {
        while let Ok(node) = self.parse() {
            match &mut self.ast {
                ASTNode::Program(prog) => {
                    prog.body.push(node)
                }
                _ => unreachable!()
            }
        }
        Ok(self)
    }

    pub fn parse(&mut self) -> Result<ASTNode> {
        let Some(token) = self.tokens.next() else {
            bail!("No more tokens");
        };
        return match &token.typ {
            TokenType::Ident => {
                error!(&token.loc, "Invalid token {}", token.data.val);
                bail!("Invalid token");
            },
            TokenType::Keyword => {
                match token.data.kw_typ.unwrap() {
                    KeywordType::Let => todo!(),
                    KeywordType::If => todo!(),
                    KeywordType::Else => todo!(),
                    KeywordType::Const => todo!(),
                    KeywordType::Fn => self.handle_function(&token),
                    KeywordType::Struct => todo!(),
                    KeywordType::Type => todo!(),
                    KeywordType::Include => todo!(),
                    KeywordType::Pub => todo!(),
                    KeywordType::Enum => todo!(),
                    KeywordType::Loop => todo!(),
                    KeywordType::While => todo!(),
                    KeywordType::For => todo!(),
                    KeywordType::Return => todo!(),
                    KeywordType::Self_ => todo!(),
                    KeywordType::True => todo!(),
                    KeywordType::False => todo!(),
                }
            }
            TokenType::Literal => todo!(),
            TokenType::Punct => todo!(),
            TokenType::Delim => todo!(),
        }
    }

    pub fn handle_function(&mut self, token: &Token) -> Result<ASTNode> {
        println!("Function");

        let mut fn_node = ast::Function {
            com: Default::default(),
            name: Default::default(),
            args: Default::default(),
            body: Default::default(),
            public: Default::default()
        };
        fn_node.com.loc = token.loc.clone();
        fn_node.name = Box::new(self.get_ident()?);

        let _ = self.expect_error(TokenType::Delim, Some(TokenData { delim_typ: Some(DelimTyp::ParenOpen), .. Default::default() }))?;
        self.handle_function_args(&mut fn_node.args)?;

        Ok(ASTNode::Function(fn_node))
    }

    pub fn handle_function_args(&mut self, args: &mut HashMap<ASTNode, ASTNode>) -> Result<()>{
        let mut paren_depth = 1;
        loop {
            let name = self.get_ident();
            if self.pop_if_next_token(TokenType::Punct, Some(TokenData { punct_typ: Some(PunctTyp::Comma), .. Default::default() })) {
                // next arg
            } else
            if self.pop_if_next_token(TokenType::Punct, Some(TokenData { punct_typ: Some(PunctTyp::Colon), .. Default::default() })) {
                // type
            }
            if let Some(token) = self.tokens.next() {
                match token.typ {
                    TokenType::Ident => todo!(),
                    TokenType::Keyword => todo!(),
                    TokenType::Literal => todo!(),
                    TokenType::Punct => todo!(),
                    TokenType::Delim => todo!(),
                }
            }
        }
        Ok(())
    }


    pub fn get_ident(&mut self) -> Result<ASTNode>{
        let ident = self.expect_error(TokenType::Ident, None)?;

        Ok(ASTNode::Ident(ast::Ident{
            com: ASTCommon::new(ident.loc.clone()),
            name: ident.data.val.clone(),
        }))
    }

    pub fn get_ast(&mut self) -> ASTNode {
        self.ast.clone()
    }

    #[allow(dead_code)]
    pub fn expect_warn(&mut self, token_typ: TokenType, data: Option<TokenData>) -> Option<&Token> {
        let Some(token) = self.tokens.next() else {
            warn!("Expected {}, but found nothing", token_typ.fmt(&data));
            return None;
        };

        match &token.typ {
            t if t == &token_typ => Some(token),
            _ => {
                warn!("Expected {}, but found nothing", token_typ.fmt(&data));
                None
            }
        }
    }


    // If the token matches it pops it from the iter
    pub fn is_next_token(&mut self, token_typ: TokenType, data: Option<TokenData>) -> bool {
        let Some(token) = self.tokens.peek() else {
            // warn!("Expected {}, but found nothing", token_typ.fmt(&data));
            return false;
        };

        match &token.typ {
            t if t == &token_typ => {
                match data {
                    Some(data) => {
                        match data {
                            _ if data.delim_typ.is_some() && token.data.delim_typ == token.data.delim_typ => (),
                            _ if data.kw_typ.is_some() && token.data.kw_typ == token.data.kw_typ => (),
                            _ if data.lit_typ.is_some() && token.data.kw_typ == token.data.kw_typ => (),
                            _ if data.punct_typ.is_some() && token.data.punct_typ == token.data.punct_typ => (),
                            _ => return false
                        }
                    },
                    None => (),
                }

                let _ = self.tokens.next();
                return true;
            },
            _ => {
                return false;
            }
        }
    }

    pub fn pop_if_next_token(&mut self, token_typ: TokenType, data: Option<TokenData>) -> bool {
        if self.is_next_token(token_typ, data) {
            self.tokens.next();
            true
        } else {
            false
        }
    }

    pub fn expect_error(&mut self, token_typ: TokenType, data: Option<TokenData>) -> Result<&Token> {
        let Some(token) = self.tokens.next() else {
            error!("Expected {}, but found nothing", token_typ.fmt(&data));
            bail!("Expected {}, but found nothing", token_typ.fmt(&data));
        };

        match &token.typ {
            t if t == &token_typ => Ok(token),
            _ => {
                error!("Expected {}, but found {}", token_typ.fmt(&data), token.to_string());
                bail!("Expected {}, but found {}", token_typ.fmt(&data), token.to_string());

            }
        }
    }
}



