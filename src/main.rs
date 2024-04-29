use std::error::Error;

use redis_starter_rust::parser;
// Uncomment this block to pass the first stage
use tokio::{
    io::{split, AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

const ADDR: &str = "127.0.0.1:6379";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind(ADDR).await?;

    println!("listening on :{}", ADDR);

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let (mut reader, mut writer) = socket.split();
            let mut buf = vec![0; 1024];

            loop {
                let n = reader.read(&mut buf).await.expect("read data error!");
                if n == 0 {
                    return;
                }

                let input = String::from_utf8_lossy(&buf[0..n]);

                let mut parser = parser::Parser::new(&input);

                let token = parser.parse().unwrap();

                dbg!(&token);
                let response = format!("{token}");
                writer.write_all(response.as_bytes()).await;
            }
        });
    }

    Ok(())
}
