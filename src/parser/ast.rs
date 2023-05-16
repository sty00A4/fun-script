use crate::location::position::Located;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueType {
    Expression, Condition
}

// < ... >
#[derive(Debug, Clone, PartialEq)]
pub enum Condition {
    True, False,
    And(Box<Located<Self>>, Box<Located<Self>>),
    Or(Box<Located<Self>>, Box<Located<Self>>),
    Not(Box<Located<Self>>),
    EQ(Located<Expression>, Located<Expression>),
    NE(Located<Expression>, Located<Expression>),
    LT(Located<Expression>, Located<Expression>),
    GT(Located<Expression>, Located<Expression>),
    LE(Located<Expression>, Located<Expression>),
    GE(Located<Expression>, Located<Expression>),
}

// ( ... )
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    ID(String), Number(f64), String(String),

    // Number
    Add(Box<Located<Self>>, Box<Located<Self>>),
    Sub(Box<Located<Self>>, Box<Located<Self>>),
    Mul(Box<Located<Self>>, Box<Located<Self>>),
    Div(Box<Located<Self>>, Box<Located<Self>>),
    Pow(Box<Located<Self>>, Box<Located<Self>>),
    Mod(Box<Located<Self>>, Box<Located<Self>>),
    Max(Box<Located<Self>>, Box<Located<Self>>),
    Min(Box<Located<Self>>, Box<Located<Self>>),

    // String
    Join(Box<Located<Self>>, Box<Located<Self>>),
    Index(Box<Located<Self>>, Box<Located<Self>>),
    SubString(Box<Located<Self>>, Box<Located<Self>>, Box<Located<Self>>),
    Contains(Box<Located<Self>>, Box<Located<Self>>),
}

// [ ... ]
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Set(Located<String>, Located<Expression>),
    SetCondition(Located<String>, Located<Condition>),
    If(Located<Condition>, Located<Block>),
    IfElse(Located<Condition>, Located<Block>, Located<Block>),
    Repeat(Located<Expression>, Located<Block>),
    While(Located<Condition>, Located<Block>),
    Loop(Located<Block>),
    Break, Next,
    Return(Located<Expression>),
    ReturnCondition(Located<Condition>)
}

// { ... }
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    statements: Vec<Statement>
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChunkTrigger {}

// |TRIGGER| BLOCK
#[derive(Debug, Clone, PartialEq)]
pub struct Chunk {
    trigger: ChunkTrigger,
    body: Block
}

// (ID)
// (ID = @EXPR)
// <ID>
// <ID = @COND>
#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    id: Located<String>,
    value: Option<Located<Expression>>
}

// (ID)
// <ID>
// (ID = EXPR)
// <ID = EXPR>
#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    id: Located<String>,
    typ: ValueType,
    default: Located<Expression>
}
// #ID PARAMS BLOCK
#[derive(Debug, Clone, PartialEq)]
pub struct Procedure {
    id: Located<String>,
    params: Vec<Located<Param>>,
    body: Block
}
// #(ID) PARAMS BLOCK
// #(ID) PARAMS EXPR
// #<ID> PARAMS BLOCK
// #<ID> PARAMS COND
#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    id: Located<String>,
    params: Vec<Located<Param>>,
    body: Block,
    return_type: ValueType
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    // variables { ... }
    variables: Vec<Located<Variable>>,
    // messages { ... }
    messages: Vec<Located<String>>,
    // chunks { ... }
    chunks: Vec<Located<Chunk>>,
    // procedures { ... }
    procedures: Vec<Located<Procedure>>,
    // functions { ... }
    functions: Vec<Located<Function>>,
}