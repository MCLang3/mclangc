use anyhow::bail;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub enum KeywordType {
    Let, If, Else, Const, Fn, Struct, Type,
    Include, Pub, Enum, Loop, While, For, 
    Return, Self_, True, False
}



impl Into<String> for KeywordType {

    fn into(self) -> String {
        match self {
            Self::Let     => String::from("let"),
            Self::If      => String::from("if"),
            Self::Else    => String::from("else"),
            Self::Const   => String::from("const"),
            Self::Fn      => String::from("fn"),
            Self::Struct  => String::from("struct"),
            Self::Type    => String::from("type"),
            Self::Include => String::from("include"),
            Self::Pub     => String::from("pub"),
            Self::Enum    => String::from("enum"),
            Self::Loop    => String::from("loop"),
            Self::While   => String::from("while"),
            Self::For     => String::from("for"),
            Self::Return  => String::from("return"),
            Self::Self_   => String::from("Self"),
            Self::True    => String::from("true"),
            Self::False   => String::from("false"),
        }
    }
}

impl ToString for  KeywordType {
    fn to_string(&self) -> String {
        (*self).into()
    }
}

impl TryInto<KeywordType> for String {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<KeywordType, Self::Error> {
        match self.as_str() {
            "let"     => Ok(KeywordType::Let),
            "if"      => Ok(KeywordType::If),
            "else"    => Ok(KeywordType::Else),
            "const"   => Ok(KeywordType::Const),
            "fn"      => Ok(KeywordType::Fn),
            "struct"  => Ok(KeywordType::Struct),
            "type"    => Ok(KeywordType::Type),
            "include" => Ok(KeywordType::Include),
            "pub"     => Ok(KeywordType::Pub),
            "enum"    => Ok(KeywordType::Enum),
            "loop"    => Ok(KeywordType::Loop),
            "while"   => Ok(KeywordType::While),
            "for"     => Ok(KeywordType::For ),
            "return"  => Ok(KeywordType::Return),
            "Self"    => Ok(KeywordType::Self_),
            "true"    => Ok(KeywordType::True),
            "false"   => Ok(KeywordType::False),
            _ => bail!("Not a keyword")
        }
    }
}