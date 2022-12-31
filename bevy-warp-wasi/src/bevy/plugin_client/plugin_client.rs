use bevy::prelude::*;
use crate::shared::*;
use std::marker::PhantomData;
use super::BoxClient;
pub struct WarpClientPlugin<A>(PhantomData<A>);

impl<A> Plugin for WarpClientPlugin<A> where A:'static  + Send + Sync {
    fn build(&self, app: &mut App) {
        app.init_resource::<Vec<NetworkEvent>>()
        .init_resource::<Option<BoxClient>>()
        .add_event::<NetworkEvent>()
        .add_event::<(ConnectionHandle,A)>()
        ;
    }
}
impl<A> Default for WarpClientPlugin<A> where A:'static  + Send + Sync {
    fn default() -> Self {
        Self(PhantomData)
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