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

    pub fn parse(&mut self) -> Result<Token> {
        match self.next() {
            Ok(Token::Array(command)) => self.unpack_command(&command),
            Err(e) => bail!("err with {e}"),
            Ok(_) => bail!("unimplemented!"),
        }
    }

    fn next(&mut self) -> Result<Token> {
        self.lexer
            .next()
            .unwrap_or_else(|| bail!("unexpected token!"))
    }

    fn unpack_command(&mut self, command: &Vec<Token>) -> Result<Token> {
        dbg!(&command);
        match command.iter().peekable().peek() {
            Some(Token::BulkString(_)) => self.handle_command(command),
            Some(_) => bail!("unexpected command!"),
            None => bail!("unexpected command!"),
        }
    }

    fn handle_command(&mut self, command: &[Token]) -> Result<Token> {
        let mut iter = command.iter().peekable();
        match iter.peek() {
            Some(Token::BulkString(cmd)) => match cmd.to_lowercase().as_str() {
                "ping" => self.ping_impl(command),
                "echo" => self.echo_impl(command),
                _ => bail!("unimplemented"),
            },
            None => bail!("unexpected token!"),
            Some(_) => bail!("unexpected token!"),
        }
    }

    fn ping_impl(&mut self, command: &[Token]) -> Result<Token> {
        let mut iter = command.iter().peekable();
        iter.next();

        match iter.next() {
            Some(Token::BulkString(response)) => Ok(Token::SimpleString(response.to_string())),
            None => Ok(Token::SimpleString("+PONG".into())),
            Some(_) => bail!("unexpected token!"),
        }
    }

    fn echo_impl(&mut self, command: &[Token]) -> Result<Token> {
        let mut iter = command.iter().peekable();
        iter.next();

        match iter.next() {
            Some(Token::BulkString(response)) => Ok(Token::SimpleString(response.to_string())),
            None => Ok(Token::BulkString("".into())),
            Some(_) => bail!("unexpected token!"),
        }
    }
}
