use async_trait::async_trait;
use eyre::{Result};
use lazy_static::lazy_static;
use futures::channel::mpsc::channel;
use futures::future::{ready};
use futures::prelude::*;
use std::borrow::Cow;
use bevy::log::{error,info};
use crate::shared::ConnectionHandle;
use crate::bevy::Client;
use crate::bevy::{event_receiver};
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use futures::future::join_all;
use std::sync::{Mutex};
use std::collections::HashMap;
lazy_static! {
    pub static ref EVENTS: Mutex<HashMap<ClientName, Vec<Vec<u8>>>> = Mutex::new(HashMap::default());
}
#[derive(Clone, Hash, Eq, PartialEq)]
pub struct ClientName(pub String);

pub struct WebSocketClient<Tx> {
    client_name: ClientName,
    command_sender: Tx,
    pub url:String,
    pub connection_handle: ConnectionHandle
}

#[async_trait]
impl<Tx> Client for WebSocketClient<Tx>
where
    Tx: Sink<Vec<u8>, Error = String> + Clone + Send + Sync + Unpin + 'static,
{
    fn sender(&self) -> Box<dyn Sink<Vec<u8>, Error = String> + Send + Sync + Unpin + 'static> {
        Box::new(self.command_sender.clone())
    }

    fn poll_once(&mut self) -> Option<Vec<Vec<u8>>> {
        let mut map = EVENTS.lock().unwrap();
        let events = map.get_mut(&self.client_name).unwrap();
        let result = events.clone();
        events.clear();
        events.truncate(10);
        return Some(result);
    }
    fn connection_handle(&self)->ConnectionHandle{
        self.connection_handle.clone()
    }
    fn client_name(&self)->ClientName{
        self.client_name.clone()
    }
}
pub async fn connect(
    client_name: ClientName,
    url: String,
) -> Result<(
    WebSocketClient<impl Sink<Vec<u8>, Error = String> + Clone + Send + Sync + Unpin + 'static
    >,pharos::Events<ws_stream_wasm::WsEvent>),
> {
    let mut meta = cross_websocket::connect(url.clone()).await?;
    let connection_handle = crate::shared::ConnectionHandle::new();
    //meta.connection_handle= connection_handle.clone();
    let _client_name_c = client_name.clone();
    let evt:pharos::Events<ws_stream_wasm::WsEvent> = meta.observe_close().await.unwrap();
    let (tx, rx)= meta.split();
    let (tx_clone, rx_clone) = channel::<Vec<u8>>(32);
    wasm_bindgen_futures::spawn_local(rx_clone.map(Ok)
      .forward(tx).map(|_|{info!("zzzz");()}));
    
    let event_receiver = event_receiver(rx);
    let result = Ok((WebSocketClient {
        client_name: client_name.clone(),
        command_sender: tx_clone.sink_map_err(|err| err.to_string()),
        url:url,
        connection_handle:connection_handle.clone(),
    },evt));
    EVENTS
    .lock()
    .unwrap()
    .insert(client_name.clone(), Vec::new());
    
    wasm_bindgen_futures::spawn_local(async {
        event_receiver.for_each(move |event| {
          //let connection_handle = crate::shared::ConnectionHandle::new();
          ready(
            // crate::shared::EVENTS
            //     .lock()
            //     .unwrap()
            //     .push(crate::shared::NetworkEvent::Message(connection_handle.clone(),event))
            EVENTS
                .lock()
                .unwrap()
                .get_mut(&client_name)
                .unwrap()
                .push(event)
          )
      }).await;
    });
    result
}

pub fn connect_websocket(clientname:String,url:String) {
    info!("connect_websocketing");
    let future_arr = vec![local_connect(ClientName(clientname),url)];
    let join_ = join_all(future_arr).then(|_l| ready(()));
    spawn_local(join_);
    info!("after connect_websocketing");
}
async fn local_connect(c: ClientName, s: String) -> () {
    connect(c.clone(), s.clone())
        .then(|cz| {
            ready(
                cz.map(|(client, mut meta)| {
                    let c_clone = c.clone();
                    let mut tx = client.sender();

                    spawn_local(async move {
                        let c = vec![];
                        tx.send(c).await.unwrap_or_else(|err| {
                            info!("err{}", err);
                        });

                        if let Some(m) = meta.next().await {
                            info!("close{:?}", m);
                            delay(1000).await;
                            local_connect(c_clone, s.clone()).await;
                        }
                    });
                    crate::bevy::CLIENTS
                        .lock()
                        .unwrap()
                        .insert(c, std::boxed::Box::new(client));
                })
                .unwrap_or_else(|err| {
                    error!("{}", err)
                }),
            )
        })
        .await
}

async fn delay(timeout_ms: i32) -> () {
    let p = js_sys::Promise::new(&mut |resolve, _| {
        let closure = Closure::wrap(Box::new(move || {
            //resolve(&42.into())
            resolve.call0(&JsValue::NULL).unwrap();
        }) as Box<dyn FnMut()>);

        set_timeout(&closure, timeout_ms);
        closure.forget();
    });
    wasm_bindgen_futures::JsFuture::from(p)
        .into_future()
        .await
        .unwrap();
    ()
}
fn set_timeout(f: &Closure<dyn FnMut()>, timeout_ms: i32) {
    window()
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            f.as_ref().unchecked_ref(),
            timeout_ms,
        )
        .expect("should register `requestAnimationFrame` OK");
}
fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}