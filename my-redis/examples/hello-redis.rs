use mini_redis::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    // open a connection with mini reddis.
    let mut client = client::connect("127.0.0.1:6379").await?;
    
    // Set a key -> value pair in mini reddis.
    client.set("key", "value".into()).await?; 
    
    // Get key value.
    let result = client.get("key").await?;
    println!("got a value from reddis {:?}", result);
    Ok(())
}
