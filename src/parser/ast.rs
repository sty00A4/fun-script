use crate::error::Error;
use crate::location::position::{Located, Position};
use crate::lexer::token::*;
use super::parser::*;

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
pub enum Trigger {}

// |TRIGGER| BLOCK
#[derive(Debug, Clone, PartialEq)]
pub struct Event {
    trigger: Trigger,
    body: Block
}

// (ID)
#[derive(Debug, Clone, PartialEq)]
pub struct Message(String);
impl Message {
    pub fn new(label: String) -> Self {
        Self(label)
    }
}
impl Parsable for Message {
    fn parse(parser: &mut Parser) -> Result<Located<Self>, crate::error::Error> {
        let Located { item: _, mut pos } = parser.token_expect(Token::ExprIn)?;
        let Located { item: token, pos: id_pos } = parser.token_check()?;
        let Token::Word(id) = token else {
            return Err(Error::new(format!("expected {}, got {}", Token::Word("".into()).name(), token.name()), parser.path.clone(), Some(id_pos)))
        };
        let Located { item: _, pos: end_pos } = parser.token_expect(Token::ExprOut)?;
        pos.extend(&end_pos);
        Ok(Located::new(Self::new(id), pos))
    }
}
// (ID)
// (ID = EXPR)
#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    id: Located<String>,
    value: Option<Located<Expression>>,
}
impl Variable {
    pub fn new(id: Located<String>, value: Option<Located<Expression>>) -> Self {
        Self { id, value }
    }
}
impl Parsable for Variable {
    fn parse(parser: &mut Parser) -> Result<Located<Self>, crate::error::Error> {
        let Located { item: _, mut pos } = parser.token_expect(Token::ExprIn)?;
        let Located { item: token, pos: id_pos } = parser.token_check()?;
        let Token::Word(id) = token else {
            return Err(Error::new(format!("expected {}, got {}", Token::Word("".into()).name(), token.name()), parser.path.clone(), Some(id_pos)))
        };
        let mut value = None;
        // if let Some(Located { item: Token::Equal, pos: _ }) = parser.token_ref() {
        //     parser.token();
        //     value = Some(Expression::parse(parser)?);
        // }
        let Located { item: _, pos: end_pos } = parser.token_expect(Token::ExprOut)?;
        pos.extend(&end_pos);
        Ok(Located::new(Self::new(Located::new(id, id_pos), value), pos))
    }
}

// (ID)
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
    messages: Vec<Located<Message>>,
    // chunks { ... }
    events: Vec<Located<Event>>,
    // procedures { ... }
    procedures: Vec<Located<Procedure>>,
    // functions { ... }
    functions: Vec<Located<Function>>,
}
impl Program {
    pub fn new() -> Self {
        Self { variables: vec![], messages: vec![], events: vec![], procedures: vec![], functions: vec![] }
    }
}
impl Parsable for Program {
    fn parse(parser: &mut Parser) -> Result<Located<Self>, crate::error::Error> {
        let mut program = Program::new();
        let mut pos = Position::default();
        while let Some(Located { item: token, pos: start_pos }) = parser.token() {
            match token {
                Token::Word(word) => match word.as_str() {
                    "data" => {
                        parser.token_expect(Token::BodyIn)?;
                        while let Some(Located { item: token, pos: _ }) = parser.token_ref() {
                            if token == &Token::BodyOut { break; }
                            program.variables.push(Variable::parse(parser)?);
                            parser.token_skip(Token::End);
                        }
                        let Located { item: _, pos: end_pos } = parser.token_expect(Token::BodyOut)?;
                        pos.extend(&end_pos)
                    }
                    "messages" => {
                        parser.token_expect(Token::BodyIn)?;
                        while let Some(Located { item: token, pos: _ }) = parser.token_ref() {
                            if token == &Token::BodyOut { break; }
                            program.messages.push(Message::parse(parser)?);
                            parser.token_skip(Token::End);
                        }
                        let Located { item: _, pos: end_pos } = parser.token_expect(Token::BodyOut)?;
                        pos.extend(&end_pos)
                    }
                    // "procedures" => {
                    //     parser.token_expect(Token::BodyIn)?;
                    //     while let Some(Located { item: token, pos: _ }) = parser.token_ref() {
                    //         if token == &Token::BodyOut { break; }
                    //         program.procedures.push(Procedure::parse(parser)?);
                    //         parser.token_skip(Token::End);
                    //     }
                    //     let Located { item: _, pos: end_pos } = parser.token_expect(Token::BodyOut)?;
                    //     pos.extend(&end_pos)
                    // }
                    // "functions" => {
                    //     parser.token_expect(Token::BodyIn)?;
                    //     while let Some(Located { item: token, pos: _ }) = parser.token_ref() {
                    //         if token == &Token::BodyOut { break; }
                    //         program.functions.push(Function::parse(parser)?);
                    //         parser.token_skip(Token::End);
                    //     }
                    //     let Located { item: _, pos: end_pos } = parser.token_expect(Token::BodyOut)?;
                    //     pos.extend(&end_pos)
                    // }
                    // "events" => {
                    //     parser.token_expect(Token::BodyIn)?;
                    //     while let Some(Located { item: token, pos: _ }) = parser.token_ref() {
                    //         if token == &Token::BodyOut { break; }
                    //         program.events.push(Event::parse(parser)?);
                    //         parser.token_skip(Token::End);
                    //     }
                    //     let Located { item: _, pos: end_pos } = parser.token_expect(Token::BodyOut)?;
                    //     pos.extend(&end_pos)
                    // }
                    word => return Err(Error::new(format!("unexpected word {word:?}"), parser.path.clone(), Some(start_pos)))
                }
                token => return Err(Error::new(format!("unexpected {}", token.name()), parser.path.clone(), Some(start_pos)))
            }
        }
        Ok(Located::new(program, pos))
    }
}