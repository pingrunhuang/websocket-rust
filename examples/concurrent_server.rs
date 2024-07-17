use mini_redis::{Connection, Frame};
use tokio::net::{TcpStream,TcpListener};
use bytes;
use std::{collections::HashMap, sync::{Arc, Mutex}};

type DB = Arc<Mutex<HashMap<String, bytes::Bytes>>>;

#[tokio::main]
async fn main(){
    let local_addr = "127.0.0.1:6379";
    let listener = TcpListener::bind(local_addr).await.unwrap();
    println!(
        "Listening at {local_addr}"
    );
    // Arc let db shared in different tasks
    let db = Arc::new(Mutex::new(HashMap::new()));
    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        let db = db.clone();
        // this will throw the task into another thread so that concurrency can be achieved
        tokio::spawn(async move {
            process(socket, db).await;
        });
        println!("Done processing request from {:?}", addr)
    }
}

async fn process(socket: TcpStream, db: DB){
    use mini_redis::Command::{self, Set, Get};

    let mut connection = Connection::new(socket);
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }           
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };
        connection.write_frame(&response).await.unwrap();
    }
}
