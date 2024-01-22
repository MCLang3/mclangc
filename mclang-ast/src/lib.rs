pub mod expression;
pub mod function;
pub mod literals;
pub mod ops;
pub mod statement;

/// Name
#[derive(PartialEq, Debug)]
pub struct Ident(pub String);


