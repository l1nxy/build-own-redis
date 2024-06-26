use std::time::Duration;

use crate::{db::DbStore, lexer::Token};

use anyhow::{bail, Result};
#[derive(Debug, Default)]
pub struct ServerState {
    db: DbStore,
    role: String,
    master_replid: String,
    master_repl_offset: u64,
}

impl ServerState {
    pub fn new(role: &str) -> Self {
        ServerState {
            db: DbStore::new(),
            role: role.to_string(),
            master_replid: String::from("8371b4fb1155b71f4a04d3e1bc3e18c4a990aeeb"),
            master_repl_offset: 0,
        }
    }
    pub fn handle_command(&mut self, command: &[Token]) -> Result<Token> {
        let mut iter = command.iter().peekable();
        match iter.peek() {
            Some(Token::BulkString(cmd)) => match cmd.to_lowercase().as_str() {
                "ping" => self.ping_impl(command),
                "echo" => self.echo_impl(command),
                "set" => self.set_impl(command),
                "get" => self.get_impl(command),
                "info" => self.info_impl(command),
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
                    if iter.peek().is_none() {
                        self.db.set(key, value, None);
                    }

                    if let Some(Token::BulkString(px)) = iter.next() {
                        if px.to_lowercase() == "px" {
                            if iter.peek().is_none() {
                                bail!("error with set")
                            } else if let Some(Token::BulkString(time)) = iter.next() {
                                let time = time.parse::<u64>()?;
                                self.db.set(key, value, Some(Duration::from_millis(time)));
                            }
                        }
                    }
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
                    Ok(Token::NullBuldString)
                }
            }
            None => Ok(Token::BulkString("".into())),
            Some(_) => bail!("unexpected token!"),
        }
    }

    fn info_impl(&mut self, command: &[Token]) -> Result<Token> {
        let mut iter = command.iter().peekable();
        iter.next();

        match iter.next() {
            Some(Token::BulkString(info_type)) => match info_type.as_str() {
                "replication" => {
                    let info = format!(
                        "role:{}\nmaster_replid:{}\nmaster_repl_offset:{}",
                        self.role, self.master_replid, self.master_repl_offset
                    );
                    Ok(Token::BulkString(info))
                }
                _ => {
                    bail!("need more para")
                }
            },
            None => Ok(Token::BulkString("".into())),
            Some(_) => bail!("unexpected token!"),
        }
    }
}
