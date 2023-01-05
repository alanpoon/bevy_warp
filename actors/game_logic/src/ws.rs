use std::{sync::{
    atomic::{AtomicUsize, Ordering},
}, net::SocketAddr};
use futures_util::{SinkExt, StreamExt, TryFutureExt};
use futures::channel::mpsc::channel;
use tokio::sync::{mpsc};
use warp::{ws::{Message, WebSocket}};
use tokio_stream::wrappers::UnboundedReceiverStream;
static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);
use crate::{user::Users};
use bevy_warp_wasi::shared::{NetworkEvent,ConnectionHandle};
use bevy_warp_wasi::bevy::plugin_server::{push_network_event,WebSocketClient};
use bevy_warp_wasi::bevy::{BoxClient,BoxClient2};
use crate::game::APP;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::collections::HashMap;
use std::boxed::Box;
lazy_static!{
    pub static ref WS_TX: Mutex<HashMap<ConnectionHandle, BoxClient2>> = Mutex::new(HashMap::new());
}
pub async fn user_connected(ws: WebSocket, users: Users,addr:Option<SocketAddr>) {
    // Use a counter to assign a new unique ID for this user.
    let my_id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);
    let client_handle = ConnectionHandle::new();
    
    eprintln!("new chat user: {}", my_id);

    // Split the socket into a sender and receive of messages.
    let (mut user_ws_tx, mut user_ws_rx) = ws.split();

    // Use an unbounded channel to handle buffering and flushing of messages
    // to the websocket...
    //let (tx, mut rx) = channel::<Vec<u8>>(32);
    let (tx, rx) = mpsc::unbounded_channel();
    let mut rx = UnboundedReceiverStream::new(rx);
    tokio::task::spawn(async move {
        while let Some(message) = rx.next().await {
            user_ws_tx
                .send(warp::ws::Message::binary(message))
                .unwrap_or_else(|e| {
                    eprintln!("websocket send error: {}", e);
                })
                .await;
        }
    });
    {
        let map = APP.clone();
        let mut m = map.lock().unwrap();
        if let Some(mut client) = m.world.get_resource_mut::<Vec<WebSocketClient>>(){
            client.push(WebSocketClient{
                connection_handle:client_handle.clone(),
                command_sender:tx.clone(),
            });
        }
        //push_network_event(NetworkEvent::Connected(client_handle.clone()),vec![1],&mut m);

    }
    

    // Save the sender in our list of connected users.
    //users.write().await.insert(my_id, tx);

    // Return a `Future` that is basically a state machine managing
    // this specific user's connection.

    // Every time the user sends a message, broadcast it to
    // all other users...
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
    user_disconnected(my_id, &users,client_handle).await;
}
pub async fn user_message(my_id: usize, msg: Message, users: &Users,connection_handle:ConnectionHandle) {
    // Skip any non-Text messages...
    let data =msg.clone().into_bytes();
    {
        let map = APP.clone();
        let mut m = map.lock().unwrap();
        push_network_event(NetworkEvent::Message(connection_handle,data),vec![1],&mut m);
    }
}

pub async fn user_disconnected(my_id: usize, users: &Users,connection_handle:ConnectionHandle) {
    eprintln!("good bye user: {}", my_id);
    {
        let map = APP.clone();
        let mut m = map.lock().unwrap();
        if let Some(mut client) = m.world.get_resource_mut::<Vec<WebSocketClient>>(){
            let index = client.iter().position(|x| x.connection_handle() == connection_handle.clone()).unwrap();
            client.remove(index);
        }
        //push_network_event(NetworkEvent::Disconnected(connection_handle),vec![1],&mut m);

    }
    // Stream closed up, so remove from the user list
    users.write().await.remove(&my_id);
}

pub static INDEX_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
    <head>
        <title>Warp Chat</title>
    </head>
    <body>
        <h1>Warp chat</h1>
        <div id="chat">
            <p><em>Connecting...</em></p>
        </div>
        <input type="text" id="text" />
        <button type="button" id="send">Send</button>
        <script type="text/javascript">
        const chat = document.getElementById('chat');
        const text = document.getElementById('text');
        const uri = 'ws://' + location.host + '/chat';
        const ws = new WebSocket(uri);

        function message(data) {
            const line = document.createElement('p');
            line.innerText = data;
            chat.appendChild(line);
        }

        ws.onopen = function() {
            chat.innerHTML = '<p><em>Connected!</em></p>';
        };

        ws.onmessage = function(msg) {
            message(msg.data);
        };

        ws.onclose = function() {
            chat.getElementsByTagName('em')[0].innerText = 'Disconnected!';
        };

        send.onclick = function() {
            const msg = text.value;
            ws.send(msg);
            text.value = '';

            message('<You>: ' + msg);
        };
        </script>
    </body>
</html>
"#;