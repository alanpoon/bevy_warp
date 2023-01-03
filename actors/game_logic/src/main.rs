mod ws;
use ws::*;
mod user;
use user::*;
mod game;
mod systems;
mod spawn_;
mod messaging_;
use tokio::sync::{mpsc, RwLock};
use std::net::SocketAddr;
use warp::Filter;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    game::init();
    let remote = warp::addr::remote()
     .map(|addr: Option<SocketAddr>| {
         println!("remote address = {:?}", addr);
     });
    let mut v =vec![];
    let users = Users::default();
    // Turn our "state" into a new Filter...
    let users = warp::any().map(move || users.clone());
    let remote = warp::addr::remote()
    ;
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
            //ws.on_upgrade(move |socket| user_connected2(socket, users,addr,app.clone()))
        });

    // GET / -> index html
    let index = warp::path::end().map(|| warp::reply::html(INDEX_HTML));

    let routes = index.or(chat);

    //let j = warp::serve(routes).run(([127, 0, 0, 1], 3031));
    let j = tokio::spawn(async move{warp::serve(routes).run(([127, 0, 0, 1], 3031)).await; });
    v.push(j);
   
    for task in v{
        task.await.unwrap_or(());
    }
    println!("Hello, world!");
}