use std::{iter::Peekable, str::Chars};

use anyhow::{bail, Result};
use thiserror::Error;

const CRLF: &str = "\r\n";

#[derive(Error, Debug)]
pub enum MyError {
    #[error("can not parse from {0}")]
    ParseError(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    SimpleString(String),
    SimpleError(String),
    Integer(String),
    BulkString(String),
    NullBuldString,
    Array(Vec<Token>),
    NullArray,
    Null,
    Boolean(bool),
    Double(String),
    BigNubmer(String),
    BulkError(String),
    Map(std::collections::HashMap<String, String>),
    Set(std::collections::HashSet<String>),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::SimpleString(s) => {
                write!(f, "+{s}{CRLF}")
            }
            Token::SimpleError(s) => {
                write!(f, "-{s}{CRLF}")
            }
            Token::Integer(n) => {
                write!(f, ":{n}{CRLF}")
            }
            Token::BulkString(s) => {
                write!(f, "${}{CRLF}{}{CRLF}", s.len(), s)
            }
            Token::NullBuldString => {
                write!(f, "$-1{CRLF}")
            }
            Token::Array(v) => {
                write!(f, "*{}{CRLF}", v.len())?;
                for i in v {
                    write!(f, "{i}")?;
                }
                write!(f, "")
            }
            Token::NullArray => {
                write!(f, "*-1{CRLF}")
            }
            Token::Null => {
                write!(f, "_{CRLF}")
            }
            Token::Boolean(b) => {
                if *b {
                    write!(f, "#t{CRLF}",)
                } else {
                    write!(f, "#f{CRLF}")
                }
            }

            Token::Double(x) => {
                write!(f, ",{x}{CRLF}")
            }

            Token::BigNubmer(n) => {
                write!(f, "({n}")
            }
            Token::BulkError(e) => {
                write!(f, "!{}{CRLF}{}{CRLF}", e.len(), e)
            }
            Token::Map(m) => {
                write!(f, "%{}{CRLF}", m.len())?;
                for (k, v) in m {
                    write!(f, "{}{CRLF}{}{CRLF}", k, v)?;
                }
                write!(f, "")
            }
            Token::Set(s) => {
                write!(f, "~{}{CRLF}", s.len())?;
                for i in s {
                    write!(f, "{i}{CRLF}")?;
                }
                write!(f, "")
            }
        }
    }
}

#[derive(Debug)]
pub struct Lexer<'a> {
    iter: Peekable<Chars<'a>>,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token>;
    fn next(&mut self) -> Option<Result<Token>> {
        match self.scanstr() {
            Ok(Some(token)) => Some(Ok(token)),
            Ok(None) => self
                .iter
                .peek()
                .map(|c| Err(MyError::ParseError(format!("can not format form {c}")).into())),
            Err(err) => Some(Err(err)),
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            iter: input.chars().peekable(),
        }
    }

    pub fn scanstr(&mut self) -> Result<Option<Token>> {
        match self.iter.peek() {
            Some('+') => self.scan_string(),
            Some(':') => self.scan_num(),
            Some('*') => self.scan_array(),
            Some('$') => self.scan_bulk_string(),
            Some(_) => Ok(None),
            None => Err(MyError::ParseError("test".into()).into()),
        }
    }

    fn next_while<F: Fn(char) -> bool>(&mut self, predicate: F) -> Option<String> {
        let mut value = String::new();
        while let Some(c) = self.next_if(&predicate) {
            value.push(c);
        }
        Some(value).filter(|v| !v.is_empty())
    }

    fn next_while_with_count<F: Fn(char) -> bool>(
        &mut self,
        predicate: F,
        count: usize,
    ) -> Option<String> {
        let mut value = String::new();
        let mut count = count;

        while count != 0 {
            if let Some(c) = self.next_if(&predicate) {
                value.push(c);
                count -= 1;
            } else {
                return None;
            }
        }
        Some(value).filter(|v| !v.is_empty())
    }

    fn next_if<F: Fn(char) -> bool>(&mut self, predicate: F) -> Option<char> {
        self.iter.peek().filter(|&c| predicate(*c))?;
        self.iter.next()
    }

    fn consume_crlf(&mut self) -> Option<()> {
        self.next_while(|c| c == '\r')?;
        self.next_while(|c| c == '\n')?;
        Some(())
    }

    fn scan_string(&mut self) -> Result<Option<Token>> {
        // consume the '+'
        self.iter.next();
        if let Some(string) = self.next_while(|c| c.is_alphanumeric()) {
            if self.consume_crlf().is_none() {
                return Err(MyError::ParseError("crlf error".into()).into());
            }
            return Ok(Some(Token::SimpleString(string)));
        }
        bail!("parse string error!")
    }

    fn scan_string_with_length(&mut self, str_length: usize) -> Result<Option<Token>> {
        if let Some(string) = self.next_while_with_count(|c| c.is_alphanumeric(), str_length) {
            self.next_while(|c| c.is_alphanumeric());
            if self.consume_crlf().is_none() {
                return Err(MyError::ParseError("crlf error".into()).into());
            }
            return Ok(Some(Token::BulkString(string)));
        }
        bail!("parse string error!")
    }

    fn scan_num(&mut self) -> Result<Option<Token>> {
        self.iter.next();
        if let Some(string) = self.next_while(|c| c.is_numeric()) {
            if self.consume_crlf().is_none() {
                return Err(MyError::ParseError("crlf error".into()).into());
            } else {
                return Ok(Some(Token::Integer(string)));
            }
        }
        bail!("parse string error!")
    }

    fn scan_array(&mut self) -> Result<Option<Token>> {
        self.iter.next(); // pass the *

        let item_count = self.next_while(|c| c.is_numeric());
        if item_count.is_none() {
            return Err(MyError::ParseError("parse array failed! no count".into()).into());
        }

        if self.consume_crlf().is_none() {
            return Err(MyError::ParseError("parse array failed! no crlf".into()).into());
        }

        let mut array = Vec::new();
        if let Some(num) = item_count {
            let count = num.parse::<usize>()?;
            for _ in 0..count {
                let token = self.scanstr()?;
                if token.is_none() {
                    return Err(MyError::ParseError("parse array failed".into()).into());
                }
                array.push(token.unwrap());
            }
        }
        Ok(Some(Token::Array(array)))
    }

    fn scan_bulk_string(&mut self) -> Result<Option<Token>> {
        self.iter.next();
        let str_length = self.next_while(|c| c.is_numeric());
        if str_length.is_none() {
            return Err(MyError::ParseError("parse array failed! no count".into()).into());
        }

        if self.consume_crlf().is_none() {
            return Err(MyError::ParseError("parse array failed! no crlf".into()).into());
        }

        if let Some(num) = str_length {
            let count = num.parse::<usize>()?;
            self.scan_string_with_length(count)
        } else {
            bail!("parse buld string error, str_length missed!")
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn test_lexer_string() {
        let mut lexer = Lexer::new("+OK\r\n:123\r\n");
        let ret = lexer.next();
        assert!(ret.is_some());
        let ret = ret.unwrap();
        assert!(ret.is_ok());
        let ret = ret.unwrap();
        assert_eq!(ret, Token::SimpleString("OK".to_string()));

        let ret = lexer.next();
        assert!(ret.is_some());
        let ret = ret.unwrap();
        assert!(ret.is_ok());
        let ret = ret.unwrap();

        assert_eq!(ret, Token::Integer("123".to_string()));
    }

    #[test]
    fn test_lexer_string_failed() {
        let mut lexer = Lexer::new("+OK\r:123\r\n");
        let ret = lexer.next();
        assert!(ret.is_some());
        let ret = ret.unwrap();
        assert!(ret.is_err());
    }

    #[test]
    fn test_lexer_number() {
        let mut lexer = Lexer::new(":123\r\n");
        let ret = lexer.next();
        assert!(ret.is_some());
        let ret = ret.unwrap();
        assert!(ret.is_ok());
        let ret = ret.unwrap();

        assert_eq!(ret, Token::Integer("123".to_string()));
    }

    #[test]
    fn test_lexer_array() {
        let mut lexer = Lexer::new("*2\r\n:123\r\n+hello\r\n");
        let ret = lexer.next();
        assert!(ret.is_some());
        let ret = ret.unwrap();
        assert!(ret.is_ok());
        let ret = ret.unwrap();

        assert_eq!(
            ret,
            Token::Array(vec![
                Token::Integer("123".into()),
                Token::SimpleString("hello".into())
            ])
        )
    }

    #[test]
    fn test_bulk_string() {
        let mut lexer = Lexer::new("$5\r\nhello\r\n");
        let ret = lexer.next();
        assert!(ret.is_some());
        let ret = ret.unwrap();
        assert!(ret.is_ok());
        let ret = ret.unwrap();

        assert_eq!(ret, Token::BulkString("hello".into()))
    }
}
