use tokio::net::{TcpListener, TcpStream};
use mini_redis::{self, Connection, Frame};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    println!("Listening on 127.0.0.1:6379");
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let sid = socket.local_addr().unwrap();
        // this will still block the main thread and wait
        // process(socket).await;
        // this will throw the task into another thread so that concurrency can be achieved
        tokio::spawn(async move{
            process(socket).await;
        });
        println!("processed socket: {:?}", sid);
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

async fn process_commands(socket: TcpStream){
}