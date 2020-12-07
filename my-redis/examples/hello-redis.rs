use mini_redis::{client, Result};
//use mini_redis::cmd::{Get, Set};
use tokio::sync::mpsc;

use bytes::Bytes;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
    },
    Set {
        key: String,
        val: Bytes,
    }
}

#[tokio::main]
pub async fn main() -> Result<()> {
    // open a connection with mini reddis.
    let mut client = client::connect("127.0.0.1:6379").await?;
    
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();
    
    // Set a key -> value pair in mini reddis.
    tokio::spawn(async move {
        let cmd = Command::Set { 
            key: "Key".to_string(), 
            val: "Val".into()
        };
        tx.send(cmd).await.unwrap();    
    });

    // Get value for key.
    tokio::spawn(async move {
        let cmd = Command::Get {key: "Key".to_string()};
        tx2.send(cmd).await.unwrap();    
    });

    while let Some(cmd) = rx.recv().await {
        match cmd {
            Command::Get {key} => {client.get(&key).await?;}
            Command::Set {key, val} => {client.set(&key, val).await?;}
        };
    };
    Ok(())
}
