use crate::{location::{path::FilePath, position::Located}, lexer::token::Token, error::Error};

pub mod parser;
pub mod ast;

use parser::Parser;
use ast::Program;

pub fn parse(path: &FilePath, tokens: Vec<Located<Token>>) -> Result<Program, Error> {
    Parser::new(path.clone(), tokens).parse()
}