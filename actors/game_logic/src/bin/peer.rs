use std::time::Duration;
use warp::Filter;

use futures_util::{ StreamExt, SinkExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use lazy_static::lazy_static;
use std::net::SocketAddr;
use std::sync::{Arc,Mutex};
use game_logic::user::Users;
use tokio::sync::mpsc;
use bevy_warp_wasi::shared::ConnectionHandle;
use tokio_stream::wrappers::UnboundedReceiverStream;
use std::{sync::{
    atomic::{AtomicUsize,Ordering},
}};
use warp::ws::WebSocket;
static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);

lazy_static!{
    pub static ref TX:Arc<Mutex<Option<mpsc::UnboundedSender<warp::ws::Message> >>> = Arc::new(Mutex::new(None));
}
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let remote = warp::addr::remote()
     .map(|addr: Option<SocketAddr>| {
         println!("remote address = {:?}", addr);
     });
    let mut v =vec![];
    let users = Users::default();
    // Turn our "state" into a new Filter...
    let users = warp::any().map(move || users.clone());
    let remote = warp::addr::remote();
    let url = "ws://127.0.0.1:3031/chat";
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    let (mut write, _read) = ws_stream.split();
    let (tx, mut rx) = mpsc::unbounded_channel::<warp::ws::Message>();
    let mut t = TX.clone();
    let mut tt = t.lock().unwrap();
    *tt = Some(tx);
    tokio::task::spawn(async move {
        while let Some(message) = rx.recv().await {
            let msg = message.into_bytes();
            let msg = Message::Binary(msg);
            write
                .send(msg)
                .await;
        }
    });
    
    // GET /chat -> websocket upgrade
    let chat = warp::path("chat")
        // The `ws()` filter will prepare Websocket handshake...
        .and(warp::ws())
        .and(users)
        .and(remote)
        .map( |ws: warp::ws::Ws, users,addr:Option<SocketAddr>| {
            // This will call our function if the handshake succeeds.
            println!("remote address = {:?}", addr);
            ws.on_upgrade(move |socket| user_connected(socket, users,addr))
        });
    let routes = chat;
    
    let j = tokio::spawn(async move{warp::serve(routes).run(([127, 0, 0, 1], 3032)).await; });
    v.push(j);
   
    for task in v{
        task.await.unwrap_or(());
    }
    

    println!("Hello, world!");
}
pub async fn user_connected(ws: WebSocket, users: Users,addr:Option<SocketAddr>) {
    // Use a counter to assign a new unique ID for this user.
    let my_id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);
    let client_handle = ConnectionHandle::new();
    
    eprintln!("new chat user: {}", my_id);

    // Split the socket into a sender and receive of messages.
    let (mut user_ws_tx, mut user_ws_rx) = ws.split();

    let (tx, rx) = mpsc::unbounded_channel::<warp::ws::Message>();
    let mut rx = UnboundedReceiverStream::new(rx);
    tokio::task::spawn(async move {
        while let Some(message) = rx.next().await {
            user_ws_tx
                .send(warp::ws::Message::binary(message))
                .await;
        }
    });
    while let Some(result) = user_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("websocket error(uid={}): {}", my_id, e);
                break;
            }
        };
        user_message(my_id, msg, &users,client_handle.clone()).await;
    }

    // user_ws_rx stream will keep processing as long as the user stays
    // connected. Once they disconnect, then...
    //user_disconnected(my_id, &users,client_handle).await;
}
async fn user_message(my_id: usize, msg: warp::ws::Message, users: &Users,client_handle:ConnectionHandle) {
    // Skip any non-Text messages...


    // New message from this user, send it to everyone else (except same uid)...
    for (&uid, tx) in users.read().await.iter() {
        if my_id != uid {
            if let Err(_disconnected) = tx.send(msg.clone()) {
                // The tx is disconnected, our `user_disconnected` code
                // should be happening in another task, nothing more to
                // do here.
            }
        }
    }
}