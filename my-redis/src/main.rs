use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};

use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(stream: TcpStream) {
    let mut conn = Connection::new(stream);
    if let Some(frame) = conn.read_frame().await.unwrap() {
        println!("Frame: {:?}", frame);

        let f_error = Frame::Error("unimplemented".to_string());
        conn.write_frame(&f_error).await.unwrap();
    } 
}
