use std::{error::Error, sync::Arc};

use clap::Parser;
use redis_starter_rust::{app::AppState, parser};
// Uncomment this block to pass the first stage
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    sync::Mutex,
};

const ADDR: &str = "127.0.0.1";

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the server port
    #[arg(long, default_value_t = 6379)]
    port: u16,

    #[arg(long, num_args = 2)]
    replicaof: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let addr = format!("{ADDR}:{}", args.port);
    let listener = TcpListener::bind(&addr).await?;

    let role = if args.replicaof.is_empty() {
        "master"
    } else {
        "slave"
    };
    let app: Arc<Mutex<AppState>> = Arc::new(Mutex::new(AppState::new(role)));

    println!("listening on :{}", addr);

    loop {
        let (mut socket, _) = listener.accept().await?;

        let app = app.clone();

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
                let mut app = app.lock().await;

                let token = parser.parse(|token| app.handle_command(token)).unwrap();

                dbg!(&token);
                let response = format!("{token}");
                writer.write_all(response.as_bytes()).await.unwrap();
            }
        });
    }

    Ok(())
}
