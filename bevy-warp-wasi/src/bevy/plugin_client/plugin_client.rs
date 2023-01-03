use bevy::prelude::*;
use crate::shared::*;
use std::marker::PhantomData;
use crate::bevy::BoxClient;
use super::www::EVENTS;
pub struct WarpClientPlugin<A>(PhantomData<A>);

impl<'d,A> Plugin for WarpClientPlugin<A> where A:'static + Send + Sync + for<'de> serde::Deserialize<'de>{
    fn build(&self, app: &mut App) {
        app.init_resource::<Vec<NetworkEvent>>()
        .init_resource::<Option<BoxClient>>()
        .add_event::<NetworkEvent>()
        .add_event::<(ConnectionHandle,A)>()
        .add_system(receive_events::<A>)
        ;
    }
}
impl<A> Default for WarpClientPlugin<A> where A:'static  + Send + Sync + for<'de> serde::Deserialize<'de>{
    fn default() -> Self {
        Self(PhantomData)
    }
}
fn receive_events<A>(
    mut client: ResMut<Option<BoxClient>>,
    mut sink: EventWriter<(ConnectionHandle,A)>,
) where A:'static + Send + Sync + for<'de> serde::Deserialize<'de> {
    if let Some(ref mut client) = *client {
        if let Some(c) = client.clients.get_mut(0){
            let connection_handle = c.connection_handle();
            if let Some(vec) = client.clients.get_mut(0).unwrap().poll_once() {
                for payload in vec {
                    match rmp_serde::decode::from_slice::<A>(&payload){
                        Ok(b)=>{
                            sink.send((connection_handle.clone(),b));
                        }
                        _=>{

                        }
                    }
                    
                }
            }
        }
       
    }
}
pub fn push_network_event(e: NetworkEvent,app:&mut App){
    match e.clone(){
        NetworkEvent::Message(_ch,msg)=>{
            if let Some(mut b)= app.world.get_resource_mut::<Vec<NetworkEvent>>(){
                b.push(e);
            }
        }
        _=>{}
    }
}