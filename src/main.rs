// use websockets::WebSocket;
use mini_redis::{client, Result};

// fn get_okex_orderbook(){
//     let endpoint:&str = "wss://ws.okx.com:8443/ws/v5/public";
//     println!("connecting to websocket endpoing: {endpoint}");
//     let mut cli = ClientBuilder::new(ENDPOINT).unwrap().connect(None).unwrap();
//     let params: &str = r#"
//     {
//         "op": "subscribe",
//         "args": [
//             {
//             "channel": "books5",
//             "instId": "BTC-USDT"
//             }
//         ]
//     }
//     "#;
//     let message = Message::text(params);
//     cli.send_message(&message).unwrap();
//     loop {

//     }
// }

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;
    client.set("hello", "world".into()).await?;
    let result = client.get("hello").await?;
    println!("Got value from the server: result={:?}", result);
    Ok(())
}
