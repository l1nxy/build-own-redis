use crate::{db::DbStore, lexer::Token};

use anyhow::{bail, Result};
#[derive(Debug, Default)]
pub struct AppState {
    db: DbStore,
}

impl AppState {
    pub fn new() -> Self {
        AppState { db: DbStore::new() }
    }
    pub fn handle_command(&mut self, command: &[Token]) -> Result<Token> {
        let mut iter = command.iter().peekable();
        match iter.peek() {
            Some(Token::BulkString(cmd)) => match cmd.to_lowercase().as_str() {
                "ping" => self.ping_impl(command),
                "echo" => self.echo_impl(command),
                "set" => self.set_impl(command),
                "get" => self.get_impl(command),
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
            Some(Token::BulkString(response)) => Ok(Token::SimpleString(response.clone())),
            None => Ok(Token::SimpleString("PONG".into())),
            Some(_) => bail!("unexpected token!"),
        }
    }

    fn echo_impl(&mut self, command: &[Token]) -> Result<Token> {
        let mut iter = command.iter().peekable();
        iter.next();

        match iter.next() {
            Some(Token::BulkString(response)) => Ok(Token::SimpleString(response.clone())),
            None => Ok(Token::BulkString("".into())),
            Some(_) => bail!("unexpected token!"),
        }
    }

    fn set_impl(&mut self, command: &[Token]) -> Result<Token> {
        let mut iter = command.iter().peekable();
        iter.next();

        match iter.next() {
            Some(Token::BulkString(key)) => {
                if let Some(Token::BulkString(value)) = iter.next() {
                    self.db.set(key, value);
                    Ok(Token::SimpleString("OK".into()))
                } else {
                    bail!("error with set")
                }
            }

            None => Ok(Token::BulkString("".into())),
            Some(_) => bail!("unexpected token!"),
        }
    }

    fn get_impl(&mut self, command: &[Token]) -> Result<Token> {
        let mut iter = command.iter().peekable();
        iter.next();

        match iter.next() {
            Some(Token::BulkString(key)) => {
                if let Some(value) = self.db.get(key) {
                    Ok(Token::SimpleString(value.clone()))
                } else {
                    Ok(Token::NullArray)
                }
            }
            None => Ok(Token::BulkString("".into())),
            Some(_) => bail!("unexpected token!"),
        }
    }
}
