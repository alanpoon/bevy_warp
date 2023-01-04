use bevy::prelude::*;
//use crate::bevy::BoxClient;
use crate::shared::*;
use std::marker::PhantomData;
use futures_util::Sink;
use std::boxed::Box;
use tokio::sync::mpsc::UnboundedSender;
//use tokio::UnboundedSender;
pub struct WarpServerPlugin<A>(PhantomData<A>);
impl<A> Plugin for WarpServerPlugin<A> where A:'static  + Send + Sync + for<'de> serde::Deserialize<'de> {
    fn build(&self, app: &mut App) {
        app.init_resource::<Vec<NetworkEvent>>()
        .init_resource::<Vec<WebSocketClient>>()
        .add_event::<NetworkEvent>()
        .add_event::<(ConnectionHandle,A)>()
        .add_system(receive_events::<A>)
        ;
    }
}
impl<A> Default for WarpServerPlugin<A> where A:'static  + Send + Sync + for<'de> serde::Deserialize<'de> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
fn receive_events<A>(
    mut network_event: ResMut<Vec<NetworkEvent>>,
    mut sink: EventWriter<(ConnectionHandle,A)>,
) where A:'static + Send + Sync + for<'de> serde::Deserialize<'de> {
    for ne in network_event.drain(..){
        match ne{
            NetworkEvent::Message(ch,data)=>{
                if let Ok(v)=rmp_serde::from_slice(&data){
                    sink.send((ch.clone(),v));
                }
            }
            _=>{}
        }
        
    }
}
pub fn push_network_event(e: NetworkEvent,update_binary:Vec<u8>,app:&mut App){
    match e.clone(){
        NetworkEvent::Message(_ch,msg)=>{
        if msg==update_binary{ //update
               app.update();
        }else{
            if let Some(mut b)= app.world.get_resource_mut::<Vec<NetworkEvent>>(){
                b.push(e);
            }
        }
        }
        _=>{}
    }
}
use crate::bevy::ClientName;
#[derive(Component)]
pub struct WebSocketClient {
    pub command_sender: UnboundedSender<Vec<u8>>,
    pub connection_handle: ConnectionHandle
}

impl  WebSocketClient
{
    pub fn sender(&self) -> UnboundedSender<Vec<u8>> {
        self.command_sender.clone()
    }
    pub fn connection_handle(&self)->ConnectionHandle{
        self.connection_handle.clone()
    }
}