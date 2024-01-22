/// field ::= '[' exp ']' '=' exp | Name '=' exp | exp
#[derive(Debug, PartialEq)]
pub enum Field {
    NameAssign(Name, Expr),
    ExprAssign(Expr, Expr),
    PosAssign(Expr),
}

/// tableconstructor ::= '{' [fieldlist] '}'
#[derive(Debug, PartialEq)]
pub struct TableConstructor(pub Vec<Field>);

/// var ::=  Name | prefixexp '[' exp ']' | prefixexp '.' Name
#[derive(Debug, PartialEq)]
pub enum Var {
    Name(Name),
    IndexExpr(IndexExpr),
    PropertyAccess(PropertyAccess),
}

/// block ::= {stat} [retstat]
#[derive(Debug, PartialEq)]
pub struct Block {
    pub stats: Vec<Stat>,
    pub retstat: Option<Vec<Expr>>,
}

/// varlist '=' explist
#[derive(Debug, PartialEq)]
pub struct Assignment {
    pub varlist: Vec<Var>,
    pub exprlist: Vec<Expr>,
}

/// local namelist ['=' explist]
#[derive(Debug, PartialEq)]
pub struct LocalAssignment {
    pub namelist: Vec<Name>,
    pub exprlist: Option<Vec<Expr>>,
}

/// for Name '=' exp ',' exp [',' exp] do block end
#[derive(Debug, PartialEq)]
pub struct ForRange {
    pub name: Name,
    pub exprs: (Expr, Expr, Option<Expr>),
    pub block: Block,
}

/// for namelist in explist do block end
#[derive(Debug, PartialEq)]
pub struct ForIn {
    pub namelist: Vec<Name>,
    pub exprlist: Vec<Expr>,
    pub block: Block,
}

/// stat ::=  ';' |
///         varlist '=' explist |
///         functioncall |
///         label |
///         break |
///         goto Name |
///         do block end |
///         while exp do block end |
///         repeat block until exp |
///         if exp then block {elseif exp then block} [else block] end |
///         for Name '=' exp ',' exp [',' exp] do block end |
///         for namelist in explist do block end |
///         function funcname funcbody |
///         local function Name funcbody |
///         local namelist ['=' explist]
#[derive(Debug, PartialEq)]
pub enum Stat {
    SemiColon,
    Assignment(Assignment), // varlist '=' explist
    FunctionCall(FunctionCall),
    Label(Name),
    Break,
    Goto(Name),
    DoBlock(Block),
    WhileBlock(WhileBlock),
    RepeatBlock(RepeatBlock),
    IfBlock(Box<IfBlock>),
    ForRange(Box<ForRange>),
    ForIn(ForIn),
    FunctionDef(FunctionDef),
    LocalFunctionDef(LocalFunctionDef),
    LocalAssignment(LocalAssignment),
}

/// funcbody ::= '(' [parlist] ')' block end
#[derive(Debug, PartialEq)]
pub struct FuncBody {
    pub params: Params,
    pub body: Block,
}

/// local function Name funcbody
#[derive(Debug, PartialEq)]
pub struct LocalFunctionDef {
    pub name: Name,
    pub body: FuncBody,
}

/// function funcname funcbody
#[derive(Debug, PartialEq)]
pub struct FunctionDef {
    pub name: FuncName,
    pub body: FuncBody,
}

/// parlist ::= namelist [',' '...'] | '...'
#[derive(Debug, PartialEq)]
pub struct Params {
    pub names: Vec<Name>,
    pub variadic: bool,
}


