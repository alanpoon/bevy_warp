use bevy::prelude::*;
use crate::shared::*;
use std::marker::PhantomData;
pub struct WarpPlugin<A>(PhantomData<A>);
impl<A> Plugin for WarpPlugin<A> where A:'static  + Send + Sync {
    fn build(&self, app: &mut App) {
        app.init_resource::<Vec<NetworkEvent>>()
        .add_event::<NetworkEvent>()
        .add_event::<(ConnectionHandle,A)>()
        ;
    }
}
impl<A> Default for WarpPlugin<A> where A:'static  + Send + Sync {
    fn default() -> Self {
        Self(PhantomData)
    }
}

pub fn push_network_event(e: NetworkEvent,update_binary:Vec<u8>,app:&mut App){
    match e.clone(){
        NetworkEvent::Message(_ch,msg)=>{
        if msg==update_binary{ //update
                println!("updating");
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