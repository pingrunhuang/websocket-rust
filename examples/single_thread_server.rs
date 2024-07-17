use tokio::net::{TcpListener, TcpStream};
use mini_redis::{self, Connection, Frame};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    println!("Listening on 127.0.0.1:6379");
    
    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        // this will still block the main thread and wait
        process(socket).await;
        println!("Done processing request from {:?}", addr);
    }
}

async fn process(socket: TcpStream) {
    let mut conn = Connection::new(socket);
    // TODO
    if let Some(frame) = conn.read_frame().await.unwrap(){
        println!("GOT: {:?}", frame);
        let resp = Frame::Error("unimplemented".to_string());
        conn.write_frame(&resp).await.unwrap()
    }
}
