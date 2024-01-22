use mclang_ast::{statement::{Block, Stat}, Ident};
use mclang_common::{token::{Token, TokenType}, Loc};
use util::{TokenIter, PeekableIterator};
mod util;

type Result<T> = anyhow::Result<T, (Loc, &'static str)>;


fn tok_typ(t: &Token) -> TokenType {
    t.typ.clone()
}


pub fn parse(tokens: &Vec<mclang_common::token::Token>) {
    let mut tokens = util::TokenIter::new(tokens);



}


pub fn parse_block(tokens: &TokenIter<'_, Token>) -> Result<mclang_ast::statement::Block> {
    let mut stats = vec![];
    while let Ok(stat) = parse_stat(tokens) {
        stats.push(stat);
    }
    // let retstat = parse_retstat(tokens).unwrap_or_default();
    Ok(Block { stats, retstat })
}


pub fn parse_stat(tokens: &TokenIter<'_, Token>) -> Result<mclang_ast::statement::Stat> {
    match tokens.peek().map(tok_typ) {
        Some(TokenType::Semi) => {
            tokens.next();
            Ok(Stat::SemiColon)
        }
        Some(TokenType::PathSep) => parse_label(tokens).map(Stat::Label),
        Some(TokenType::Break) => {
            tokens.next();
            Ok(Stat::Break)
        }
        Some(TokenType::Goto) => {
            let cur = tokens.next();
            if let Some(TokenType::Ident{ val: name}) = tokens.next().map(tok_typ) {
                Ok(Stat::Goto(Ident(name.to_string())))
            } else {
                Err((cur.into(), "Invalid goto statement"))
            }
        }
        Some(TokenType::While) => parse_while_block(tokens).map(Stat::WhileBlock),
        Some(TokenType::Loop) => parse_loop_block(tokens).map(Stat::LoopBlock),
        Some(TokenType::If) => parse_if_block(tokens).map(|f| Stat::IfBlock(Box::new(f))),
        Some(TokenType::For) => {
            tokens.next();
            tokens.next();
            if let Some(TokenType::Assign) = tokens.peek().map(to_kind) {
                tokens.prev();
                tokens.prev();
                parse_for_range(tokens).map(|f| Stat::ForRange(Box::new(f)))
            } else {
                tokens.prev();
                tokens.prev();
                parse_for_in(tokens).map(Stat::ForIn)
            }
        }
        Some(TokenType::Function) => parse_function_def(tokens).map(Stat::FunctionDef),
        Some(TokenType::Local) => {
            tokens.next();
            if let Some(TokenType::Function) = tokens.peek().map(to_kind) {
                tokens.prev();
                parse_local_function_def(tokens).map(Stat::LocalFunctionDef)
            } else {
                tokens.prev();
                parse_local_assignment(tokens).map(Stat::LocalAssignment)
            }
        }
        Some(TokenType::Ident(ident)) => {
            tokens.next();
            if let Some(TokenType::LParen) = tokens.peek().map(to_kind) {
                Ok(Stat::FunctionCall(FunctionCall {
                    expr: Box::new(PrefixExpr::Var(Var::Name(Name(ident.to_string())))),
                    args: parse_args(tokens)?,
                }))
            } else {
                tokens.prev();
                parse_assignment(tokens).map(Stat::Assignment)
            }
        }
        _ => Err(()),
    }
}

