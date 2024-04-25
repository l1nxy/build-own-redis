use std::error::Error;

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

                let data: Vec<&str> = input.split("\r\n").filter(|s| !s.is_empty()).collect();
                println!("{:?}", data);
                let param_count_str = data.first().unwrap();
                if !param_count_str.starts_with('*') {
                    return;
                }

                let param_count = param_count_str[1..].parse::<usize>().unwrap();
                println!("{param_count}");

                for i in 0..param_count {
                    println!("{i}");
                    let index = 1 + i;
                    let command = data[index + 1];
                    if command == "ping" {
                        writer
                            .write_all("+PONG\r\n".as_bytes())
                            .await
                            .expect("write data error!");
                    }
                }
            }
        });
    }

    Ok(())
}

#[derive(Debug)]
enum COMMANDTYPE {
    PING,
}

#[derive(Debug)]
struct RCommand {
    command_type: COMMANDTYPE,
    command_content: String,
}

impl RCommand {}
