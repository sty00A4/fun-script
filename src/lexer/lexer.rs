use crate::{location::{position::*, path::FilePath}, error::Error};
use super::token::Token;

pub struct Lexer {
    pub path: FilePath,
    text: String,
    idx: usize, ln: usize, col: usize
}
impl Lexer {
    pub fn new(path: FilePath, text: String) -> Self {
        Self { path, text, idx: 0, ln: 0, col: 0 }
    }
    pub fn get(&self) -> Option<char> {
        self.text.get(self.idx..self.idx+1)?.chars().next()
    }
    pub fn pos(&self) -> Position {
        Position::new(self.ln..self.ln+1, self.col..self.col+1)
    }
    pub fn advance(&mut self) {
        if self.get() == Some('\n') {
            self.ln += 1;
            self.col = 0;
        } else {
            self.col += 1;
        }
        self.idx += 1;
    }

    pub fn next(&mut self) -> Result<Option<Located<Token>>, Error> {
        while let Some(c) = self.get() {
            if !c.is_whitespace() { break; }
            self.advance();
        }
        let mut pos = self.pos();
        match self.get() {
            Some(c) => match c {
                '(' => {
                    self.advance();
                    Ok(Some(Located::new(Token::ExprIn, pos)))
                }
                ')' => {
                    self.advance();
                    Ok(Some(Located::new(Token::ExprOut, pos)))
                }
                '"' => {
                    self.advance();
                    let mut string = String::new();
                    while let Some(c) = self.get() {
                        if c == '"' { break; }
                        string.push(c);
                        self.advance();
                    }
                    if self.get() != Some('"') {
                        return Err(Error::new(format!("unclosed string"), self.path.clone(), Some(self.pos())))
                    }
                    pos.extend(&self.pos());
                    self.advance();
                    Ok(Some(Located::new(Token::String(string), pos)))
                }
                c if c.is_digit(10) => {
                    let mut number = String::from(c);
                    self.advance();
                    while let Some(c) = self.get() {
                        if !c.is_digit(10) { break; }
                        number.push(c);
                        pos.extend(&self.pos());
                        self.advance();
                    }
                    if self.get() == Some('.') {
                        number.push('.');
                        pos.extend(&self.pos());
                        self.advance();
                        while let Some(c) = self.get() {
                            if !c.is_digit(10) { break; }
                            number.push(c);
                            pos.extend(&self.pos());
                            self.advance();
                        }
                    }
                    match number.parse() {
                        Ok(number) => Ok(Some(Located::new(Token::Number(number), pos))),
                        Err(err) => Err(Error::new(format!("error while parsing number {number:?}: {err}"), self.path.clone(), Some(pos)))
                    }
                }
                c if c.is_alphabetic() => {
                    let mut word = String::from(c);
                    self.advance();
                    while let Some(c) = self.get() {
                        if !c.is_alphanumeric() && c != '_' { break; }
                        word.push(c);
                        pos.extend(&self.pos());
                        self.advance();
                    }
                    Ok(Some(Located::new(Token::from_word(word), pos)))
                }
                _ => todo!("lex token, char {c:?}")
            }
            None => Ok(None)
        }
    }
    pub fn lex(&mut self) -> Result<Vec<Located<Token>>, Error> {
        let mut tokens = vec![];
        while let Some(token) = self.next()? {
            tokens.push(token);
        }
        Ok(tokens)
    }
}

