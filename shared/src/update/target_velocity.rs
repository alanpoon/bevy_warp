use bevy_rapier2d::prelude::*;
use bevy::prelude::*;
use crate::*;
pub fn velocity(mut v:&mut Velocity,tv:Vec2 ){
    let f = if tv.x * tv.x+tv.y * tv.y>=2.0{
        1.0
    } else{
        std::f32::consts::SQRT_2
    };
    v.linvel.x = tv.x *50.0 * f;
    v.linvel.y = tv.y * 50.0 * f;
}
