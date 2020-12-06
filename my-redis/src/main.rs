use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Command, Connection, Frame};
use mini_redis::Command::{Get, Set};
use bytes::Bytes;

use std::io;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type Db = Arc<Mutex<HashMap<String, Bytes>>>; 

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    let db:Db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let clone_db = db.clone();
        tokio::spawn(async move {
            process(socket, clone_db).await;
        });
    }
}

async fn process(stream: TcpStream, db: Db) {
    let mut conn = Connection::new(stream);

    while let Some(frame) = conn.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }            
            }
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };
        println!("{:?}", &response);
        conn.write_frame(&response).await.unwrap();
    } 
}
