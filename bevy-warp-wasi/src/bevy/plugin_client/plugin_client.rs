use bevy::prelude::*;
use crate::shared::*;
use std::marker::PhantomData;
use crate::bevy::BoxClient;
use super::www::EVENTS;
use super::ClientName;
pub struct WarpClientPlugin<A>(PhantomData<A>);

impl<A> Plugin for WarpClientPlugin<A> where A:'static + Send + Sync  +  for<'de> serde::Deserialize<'de>{
    fn build(&self, app: &mut App) {
        app.init_resource::<Vec<NetworkEvent>>()
        .init_resource::<Option<BoxClient>>()
        .add_event::<NetworkEvent>()
        .add_event::<(ConnectionHandle,A)>()
        .add_system(receive_events::<A>)
        ;
    }
}
impl<A> Default for WarpClientPlugin<A>{
    fn default() -> Self {
        Self(PhantomData)
    }
}
pub struct WarpClientPlugin2<A,B>(PhantomData<A>,PhantomData<B>);

impl<A,B> Plugin for WarpClientPlugin2<A,B> where A:'static + Send + Sync  +  for<'de> serde::Deserialize<'de>,
B:'static + Send + Sync  +  for<'de> serde::Deserialize<'de>{
    fn build(&self, app: &mut App) {
        app.init_resource::<Vec<NetworkEvent>>()
        .init_resource::<Option<BoxClient>>()
        .add_event::<NetworkEvent>()
        .add_event::<(ConnectionHandle,A)>()
        .add_system(receive_events::<A>)
        .add_event::<(ConnectionHandle,B)>()
        .add_system(receive_events::<B>)
        ;
    }
}
impl<A,B> Default for WarpClientPlugin2<A,B>{
    fn default() -> Self {
        Self(PhantomData,PhantomData)
    }
}
fn receive_events<A>(
    mut client: ResMut<Option<BoxClient>>,
    mut sink: EventWriter<(ConnectionHandle,A)>,
) where A:'static + Send + Sync + for<'de> serde::Deserialize<'de> {
    if let Some(ref mut client) = *client {
        let mut found = None;
        for c in client.clients.iter_mut(){
            //info!("{}",std::any::type_name::<A>().to_string());
            if c.client_name()==ClientName(std::any::type_name::<A>().to_string()){
                found = Some(c);
                //info!("found {:?}",std::any::type_name::<A>().to_string());
                break;
            }
        }
        if let Some(ref mut client) = found{
            let connection_handle = client.connection_handle();
            if let Some(vec) = (*client).poll_once() {
                for payload in vec {
                    match rmp_serde::decode::from_slice::<A>(&payload){
                        Ok(b)=>{
                            sink.send((connection_handle.clone(),b));
                        }
                        Err(e)=>{
                            info!("error {:?}",e);
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