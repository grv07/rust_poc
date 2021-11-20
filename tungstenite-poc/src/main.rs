use tokio;
use futures_util::StreamExt;
use tungstenite::protocol::Message;
use futures_util::sink::SinkExt;
use tokio_tungstenite::connect_async;

fn get_url() -> String {
    String::from("ws://localhost:6789")
}

#[tokio::main]
async fn main() {
    let url = get_url();
    let (mut stream, resp) = connect_async(url).await.expect("some");
    println!("{:?}", resp);
    stream.send(Message::Text("5".to_string())).await;
    let data = stream.next().await;
    println!("{:?}", data);
}

