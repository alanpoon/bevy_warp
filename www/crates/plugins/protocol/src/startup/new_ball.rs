use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use shared::*;
use protocol::Command;
use rand::{thread_rng, Rng};
pub fn new_ball(mut cmd:Commands,mut commands: ResMut<protocol::Commands>,mut local_user_info: ResMut<LocalUserInfo>){
    info!("sending welcome");
    let tv_= rmp_serde::to_vec(&ClientMessage::SubWelcome).unwrap(); 
    commands.push(Command::WS(tv_));
    let mut rng = thread_rng();
    let n: u32 = rng.gen_range(100000..999999);
    let tv = ClientMessage::Welcome{
        game_id:String::from("hello"),
        ball_id:BallId(n),
        ball_label:BallLabel(n.to_string(),String::from("")),
    };
    let tv_= rmp_serde::to_vec(&tv).unwrap(); 
    //pub tv
    commands.push(Command::WS(tv_));
    //sub game_logic
    //sub game_logic_specify.${n}
    //sub peer

    *local_user_info = LocalUserInfo(UserInfo{
        ball_id:BallId(n)
    });
}