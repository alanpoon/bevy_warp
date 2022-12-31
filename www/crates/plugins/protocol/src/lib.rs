//#[cfg(target_arch = "wasm32")]
mod wasm;
//#[cfg(target_arch = "wasm32")]
use wasm::*;

mod c_;
mod msg_handler;
#[cfg(not(target_arch = "wasm32"))]
mod native;
mod system;
mod startup;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_warp_wasi::bevy::{set_client,connect_websocket,BoxClient};
#[cfg(not(target_arch = "wasm32"))]
use native::*;
#[derive(SystemLabel, PartialEq, Eq, Debug, Hash, Clone)]
pub enum ProtocolSystem {
    ReceiveEvents,
    HandleEvents,
    SendCommands,
    ConnectWebSocket
}

use chrono::prelude::*;
use futures::prelude::*;
use protocol::{Command, Event};
use serde::{Deserialize, Serialize};
use tracing::error;
use wasm_bindgen::prelude::wasm_bindgen;
pub struct ProtocolPlugin;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = push_web_bevy_events_fn)]
    fn push_web_bevy_events_fn(msg: &str, msg_ago: &str, user: &str);
    #[wasm_bindgen(js_namespace = window, js_name = push_web_bevy_events_fn2)]
    fn push_web_bevy_events_fn2(msg: &str);
}

use shared::*;
impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        let app = app
            .init_resource::<protocol::Commands>()
            .init_resource::<protocol::Events>()
            .init_resource::<Option<BoxClient>>()
            .init_resource::<LocalUserInfo>()
            //.add_system(receive_events.label(ProtocolSystem::ReceiveEvents))
            .add_system(
                handle_events
                    .label(ProtocolSystem::HandleEvents)
                    .after(ProtocolSystem::ReceiveEvents)
                    .before(ProtocolSystem::SendCommands),
            )
            //.add_system(system::camera::move_with_local_player)
            .add_system(
                send_commands
                    .label(ProtocolSystem::SendCommands)
                    .after(ProtocolSystem::ReceiveEvents),
            );
        app.add_startup_system(connect_websocket.label(ProtocolSystem::ConnectWebSocket))
        .add_startup_system(startup::new_ball::new_ball.after(ProtocolSystem::ConnectWebSocket));
        #[cfg(target_arch = "wasm32")]
        app.add_system(set_client);

    }
}

fn handle_events(
    mut cmd: Commands,
    mut balls: Query<(&BallId, &Transform, &mut Velocity)>,
    mut commands: ResMut<protocol::Commands>,
    mut events: ResMut<protocol::Events>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    gamepads: Res<Gamepads>,
    button_inputs: Res<Input<GamepadButton>>,
    local_user_info: Res<LocalUserInfo>,
    axes: Res<Axis<GamepadAxis>>,
) {

    let ref mut e = *events;
    e.clear();
    e.truncate(); //added
    let mut target_velocity_x = 0.0;
    let mut target_velocity_y = 0.0;
    let mut pressed = false;
    if keyboard_input.just_pressed(KeyCode::Left) {
        if keyboard_input.pressed(KeyCode::Up) {
            target_velocity_y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            target_velocity_y -= 1.0;
        }
        target_velocity_x -= 1.0;
        pressed = true;
    }
    if keyboard_input.just_pressed(KeyCode::Right) {
        if keyboard_input.pressed(KeyCode::Up) {
            target_velocity_y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            target_velocity_y -= 1.0;
        }
        target_velocity_x += 1.0;
        pressed = true;
    }
    if keyboard_input.just_pressed(KeyCode::Up) {
        if keyboard_input.pressed(KeyCode::Left) {
            target_velocity_x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            target_velocity_x += 1.0;
        }
        target_velocity_y += 1.0;
        pressed = true;
    }
    if keyboard_input.just_pressed(KeyCode::Down) {
        if keyboard_input.pressed(KeyCode::Left) {
            target_velocity_x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            target_velocity_x += 1.0;
        }
        target_velocity_y -= 1.0;
        pressed = true;
    }
    keyboard_input.clear();

    if pressed {
        for (ball_id_ingame, t, mut v) in balls.iter_mut() {
            let ball_id = (*local_user_info).0.ball_id;
            if ball_id_ingame == &ball_id {
                let mut send = false;
                if target_velocity_x != 0.0 {
                    if v.linvel.x / target_velocity_x < 0.0 {
                        send = true;
                    } else if v.linvel.x == 0.0 {
                        send = true;
                    }
                }
                if target_velocity_y != 0.0 {
                    if v.linvel.y / target_velocity_y < 0.0 {
                        send = true;
                    } else if v.linvel.y == 0.0 {
                        send = true;
                    }
                }
                if send {
                    info!(
                        "send target_velocity {:?} {:?}",
                        target_velocity_x, target_velocity_y
                    );
                    let c = c_::target_velocity(
                        ball_id,
                        target_velocity_x,
                        target_velocity_y,
                        &mut v,
                    );
                    for c in c {
                        (*commands).push(c);
                    }
                }

                break;
            }
        }
    }
    
}
fn handle_client_op(){
    
}
use futures::future::ready;
fn send_commands(
    mut cmd: Commands,
    mut client: ResMut<Option<BoxClient>>,
    mut commands: ResMut<protocol::Commands>,
    mut _events: ResMut<protocol::Events>,
) {
    if let Some(ref mut client) = *client {
        for command in commands.iter() {
            let command = command.clone();
            let len = client.clients.len();
            let rand_int = get_random_int(0, len as i32);
            let mut sender = client.clients.get_mut(rand_int).unwrap().sender();
            match command {
                Command::WS(b)=>{
                    let b_clone = b.clone();
                    block_on(async move {
                        sender
                            .send(b)
                            .await
                            .unwrap_or_else(|err| {
                                error!("{}", err);
                            });
                        ready(b_clone)
                    });
                }
                Command::StoreLocal(user_info) => {
                    let local_user_info = LocalUserInfo(user_info);
                    cmd.insert_resource(local_user_info);
                }
                _ => {}
            }
        }
        commands.clear();
    }
}
// fn receive_events(
//     mut cmd: Commands,
//     mut client: ResMut<Option<BoxClient>>,
//     mut events: ResMut<protocol::Events>,
//     mut set: ParamSet<(
//         Query<(Entity, &BallId, &mut Transform, &mut Velocity), With<BallId>>,
//         // also access the whole world ... why not
//         //&World,
//     )>,
//     mut to_despawn: ResMut<EntityToRemove>,
//     local_user_info: Res<LocalUserInfo>,
//     asset_server: Res<AssetServer>,
// ) {
//     if let Some(ref mut client) = *client {
//         let len = client.clients.len();
//         let _rand_int = get_random_int(0, len as i32);
//         if let Some(vec) = client.clients.get_mut(0).unwrap().poll_once() {
//             for event in vec {
//                 //if let Event::Nats(_client_name,s_op)=handle_server_op(event.clone()).unwrap(){
//                 let s_op = handle_server_op(event);
//                 match s_op {
//                     Ok(t) => {
//                         match t.clone() {
//                             nats::proto::ServerOp::Msg {
//                                 subject,
//                                 sid: _,
//                                 reply_to: _,
//                                 payload,
//                             } => {
//                                 if subject.contains("game_logic") {
//                                     //if subject == String::from("game_logic"){
//                                     let server_message: ServerMessage =
//                                         rmp_serde::from_slice(&payload).unwrap();
//                                     match server_message {
//                                         ServerMessage::TargetVelocity {
//                                             ball_id,
//                                             target_velocity,
//                                         } => {
//                                             //for (entity, qball_id,mut tv) in query.iter_mut(){
//                                             info!("receive {:?} tv {:?}", ball_id, target_velocity);
//                                             msg_handler::target_velocity::_fn(
//                                                 &mut cmd,
//                                                 &mut set,
//                                                 ball_id,
//                                                 target_velocity,
//                                             );
//                                         }

//                                         ServerMessage::GameState {
//                                             ball_bundles,
//                                             ..
//                                         } => {
                                            
//                                             msg_handler::game_state::_fn_spawn_or_update_ball_bundles(&mut cmd,&mut set,ball_bundles);
//                                         }
//                                         _ => {}
//                                     }

//                                     continue;
//                                 } else if subject.contains("welcome") {
//                                     let server_message: ServerMessage =
//                                         rmp_serde::from_slice(&payload).unwrap();
//                                     match server_message {
//                                         ServerMessage::Welcome { ball_bundle } => {
//                                             info!("welcome_ ball_bundle {:?}", ball_bundle.clone());
//                                             cmd.spawn_bundle(ball_bundle.clone());
//                                             if ball_bundle.ball_id == (*local_user_info).0.ball_id {
//                                             }
//                                         }
//                                         _ => {}
//                                     }
//                                 }
//                             }
//                             _ => {}
//                         }
//                         let event = Event::Nats(String::from("some_client"), t);
//                         events.push(event);
//                     }
//                     Err(e) => {
//                         info!("protocol e {:?}", e);
//                     }
//                 }
//             }
//         }
//     }
// }
