use std::f32::consts::PI;

use crate::{structures::{Pos, ViewAngles}, log};


pub fn calc_angle(root: Pos, target: Pos) -> ViewAngles{



    let delta = Pos {
        x: root.x - target.x,
        y: root.y - target.y,
        z: root.z - target.z
    };
    
    let hyp = f32::sqrt(delta.x.powi(2) + delta.y.powi(2) + delta.z.powi(2));

    let yaw =  f32::atan2(delta.y,delta.x) * 180f32/PI + 270f32;
    let pitch = f32::acos(delta.z/hyp) * 180f32/PI - 90f32;

    ViewAngles{ pitch, yaw }


}
