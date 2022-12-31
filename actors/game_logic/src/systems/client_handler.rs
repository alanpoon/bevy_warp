use bevy::prelude::*;
use bevy_warp_wasi::shared::{ConnectionHandle};
use shared::*;
pub fn listen_for_events(mut evs: EventReader<(ConnectionHandle,ClientMessage)>) {
    for  (ch,cm) in evs.iter() {
        println!("received DummyEvent from  {:?} {:?}", ch,cm);
    }
}