use std::collections::HashMap;

use crate::common::Loc;
use super::attributes::ASTAttr;


#[derive(Debug, Clone, Default)]
pub struct ASTCommon {
    pub loc: Loc,
    pub attr: HashMap<String, ASTAttr>,
}

#[allow(unused)]
impl ASTCommon {
    pub fn new(loc: Loc) -> Self {
        Self {
            loc,
            attr: HashMap::new()
        }
    }
    pub fn add_attr(&mut self, left: String, right: ASTAttr) -> &mut Self {
        self.attr.insert(left, right);
        self
    }
}