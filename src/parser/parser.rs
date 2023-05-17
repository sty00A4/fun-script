use crate::{location::{path::FilePath, position::Located}, lexer::token::Token, error::Error, join};

pub struct Parser {
    pub path: FilePath,
    tokens: Vec<Located<Token>>
}
impl Parser {
    pub fn new(path: FilePath, tokens: Vec<Located<Token>>) -> Self {
        Self { path, tokens }
    }

    pub fn token(&mut self) -> Option<Located<Token>> {
        if self.tokens.len() > 0 { Some(self.tokens.remove(0)) } else { None }
    }
    pub fn token_ref(&self) -> Option<&Located<Token>> {
        self.tokens.get(0)
    }
    pub fn token_check(&mut self) -> Result<Located<Token>, Error> {
        let Some(token) = self.token() else {
            return Err(Error::new("unexpected end of input", self.path.clone(), None))
        };
        Ok(token)
    }
    pub fn token_expect(&mut self, expect: Token) -> Result<Located<Token>, Error> {
        let token = self.token_check()?;
        if token.item != expect {
            return Err(Error::new(format!("expected {}, got {}", expect.name(), token.item.name()), self.path.clone(), Some(token.pos)))
        }
        Ok(token)
    }
    pub fn token_expects(&mut self, expects: Vec<Token>) -> Result<Located<Token>, Error> {
        let token = self.token_check()?;
        if expects.contains(&token.item) {
            return Err(Error::new(format!("expected {}, got {}", join!(expects, "/"), token.item.name()), self.path.clone(), Some(token.pos)))
        }
        Ok(token)
    }
    pub fn token_skip(&mut self, expect: Token) {
        let Some(token) = self.token_ref() else { return };
        if token.item == expect {
            self.token();
        }
    }
}

pub trait Parsable where Self: Sized {
    fn parse(parser: &mut Parser) -> Result<Located<Self>, Error>;
    fn can_parse(parser: &mut Parser) -> Option<Located<Self>> {
        None
    }
}