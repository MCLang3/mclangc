use std::cmp::max;

use mclang_common::token::{Token, TokenType};
use anyhow::Result;

pub trait PeekableIterator: Iterator {
    fn peek(&mut self) -> Option<Self::Item>;
}

pub trait ForwardBackwardIterator: Iterator {
    fn prev(&mut self) -> Option<Self::Item>;
}

#[derive(Debug)]
pub struct TokenIter<'a, Item>
where
    Item: 'a + PartialEq,
{
    index: Option<usize>,
    vector: &'a [Item],
}

impl<'a, Item> TokenIter<'a, Item>
where
    Item: PartialEq,
{
    pub fn new(vector: &'a [Item]) -> TokenIter<'a, Item> {
        TokenIter {
            index: None,
            vector,
        }
    }
}

impl<'a, Item> PeekableIterator for TokenIter<'a, Item>
where
    Item: PartialEq,
{
    fn peek(&mut self) -> Option<&'a Item> {
        let index = match self.index {
            Some(i) => i + 1,
            None => 0,
        };

        self.vector.get(index)
    }
}

impl<'a, Item> Iterator for TokenIter<'a, Item>
where
    Item: PartialEq,
{
    type Item = &'a Item;

    fn next(&mut self) -> Option<&'a Item> {
        let index = match self.index {
            Some(i) => i + 1,
            None => 0,
        };

        self.index = Some(index);
        self.vector.get(index)
    }
}

impl<'a, Item> ForwardBackwardIterator for TokenIter<'a, Item>
where
    Item: PartialEq,
{
    fn prev(&mut self) -> Option<&'a Item> {
        let index = match self.index {
            None => return None,
            Some(0) => {
                self.index = None;
                return None;
            }
            Some(i) => max(i - 1, 0),
        };

        self.index = Some(index);
        self.vector.get(index)
    }
}

impl<'a> TokenIter<'a, Token> {
    pub fn assert_next(&mut self, knd: &TokenType) -> Result<(), ()> {
        match self.next() {
            Some(Token { typ, .. }) => {
                if typ == knd {
                    Ok(())
                } else {
                    Err(())
                }
            }
            None => Err(()),
        }
    }
    pub fn cur(&mut self) -> Option<&'a Token> {
        let index = match self.index {
            None => return None,
            Some(i) => i,
        };
        self.vector.get(index)
    }
}


pub fn bin_priority(op: &Option<TokenType>) -> i32 {
    match op {
        Some(TokenType::Star)  | Some(TokenType::Slash) |
        Some(TokenType::Percent) => 10,
        
        Some(TokenType::Plus)  | Some(TokenType::Minus) => 9,
        Some(TokenType::Shl)   | Some(TokenType::Shl) => 7,
        Some(TokenType::And)   => 6,
        Some(TokenType::Caret) => 5,
        Some(TokenType::Or)    => 4,

        Some(TokenType::Lt) | Some(TokenType::Gt) |
        Some(TokenType::Le) | Some(TokenType::Ge) |
        Some(TokenType::Eq) | Some(TokenType::Ne) => 3,

        Some(TokenType::OrOr) | Some(TokenType::AndAnd) => 1,
        _ => 0,
    }
}

const UNARY_PRIORITY: i32 = 12;
