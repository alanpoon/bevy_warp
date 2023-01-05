use futures_util::{ StreamExt, SinkExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use std::time::Duration;
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let url = "ws://127.0.0.1:3031/chat";
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");

    let (mut write, _read) = ws_stream.split();
    let mut c=0;
    loop{
        tokio::time::sleep(Duration::from_millis(30)).await;
        write.send(Message::Binary(vec![1])).await.unwrap_or(());
    }
}