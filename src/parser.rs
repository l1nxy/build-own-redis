use anyhow::bail;

use crate::lexer::{Lexer, Token};
use anyhow::Result;

#[derive(Debug)]
pub struct Parser<'a> {
    lexer: std::iter::Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser {
            lexer: Lexer::new(input).peekable(),
        }
    }

    pub fn parse<T>(&mut self, mut deal_fn: T) -> Result<Token>
    where
        T: FnMut(&[Token]) -> Result<Token>,
    {
        match self.next() {
            Ok(Token::Array(command)) => deal_fn(&command),
            Err(e) => bail!("err with {e}"),
            Ok(_) => bail!("unimplemented!"),
        }
    }

    fn next(&mut self) -> Result<Token> {
        self.lexer
            .next()
            .unwrap_or_else(|| bail!("unexpected token!"))
    }
}
