use crate::{location::{path::FilePath, position::Located}, lexer::token::Token, error::Error};

use super::ast::*;

pub struct Parser {
    pub path: FilePath,
    tokens: Vec<Located<Token>>
}
impl Parser {
    pub fn new(path: FilePath, tokens: Vec<Located<Token>>) -> Self {
        Self { path, tokens }
    }

    pub fn parse(&mut self) -> Result<Program, Error> {
        todo!("parse")
    }
}