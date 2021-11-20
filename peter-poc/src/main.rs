use awc::{Client, ClientBuilder, Connector};
//use actix_http::client::Connector;
use futures_util::{sink::SinkExt, stream::StreamExt};
use std::error::Error as StdError;
use std::sync::Arc;

async fn get_client_config() -> rustls::client::ClientConfig {
    let root_certs = rustls::RootCertStore::empty();
    rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_certs)
        .with_no_client_auth()
}

//async fn get_connector() -> Connector<T, U> {
//    let client_config = get_client_config().await;
//    Connector::new().rustls(Arc::new(client_config))
//}

async fn get_client() -> Client {
    let client_config: rustls::client::ClientConfig = get_client_config().await;
    let connector = Connector::new().rustls(client_config).finish();
    ClientBuilder::new().connector(connector).finish()
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    let (_resp, mut connection) = awc::Client::new()
        .ws("ws://localhost:6789")
        .connect()
        .await?;
    println!("{:?}", connection);
    connection
        .send(awc::ws::Message::Text("Echo".to_string()))
        .await?;
    let response = connection.next().await.unwrap()?;
    println!("{:?}", response);
    Ok(())
}
