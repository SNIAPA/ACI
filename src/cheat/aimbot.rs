use super::{Cheat, CheatModule};
use crate::{structures::{ent::Ent, Pos, ViewAngles}, util::{Result, error::CheatError}, mem::follow_offsets, offsets::{PLAYER_COUNT, PLAYER_LIST}, log, dbg};
use std::{f32::consts::PI, sync::{Mutex, Arc, RwLock}, thread, time::Duration, panic::catch_unwind};

static DEFAULT_AIM_FOV: f32 = 360.0;

pub struct Aimbot {
    last_target: *mut Ent,
    aim_fov: f32,
}

impl CheatModule for Aimbot {
    unsafe fn run(&self, writable: &mut Cheat) -> Result<()>{
        

        let player_count = *writable.player_count;
        if player_count == 0 {
            return Ok(())
        };
        
        let local_player = *writable.local_player;
        let players = &mut (*writable.get_players()).clone().to_vec()[..player_count as usize-1];

        players.sort_by(|&a, &b| {
            (a.read_unaligned())
                .pos
                .dist(&local_player.pos)
                .total_cmp(
                    &(b.read_unaligned()).pos.dist(&local_player.pos)
                )
        });


        if let Some(target) = players.iter().copied().find(|&x| {
            let view_angles = Self::calc_angle(local_player.pos.clone(), x.read().pos);
            x.read_unaligned().hp <= 100
                && ((view_angles.yaw - local_player.view_angles.yaw).abs()
                    + (view_angles.pitch - local_player.view_angles.pitch).abs()
                    < self.aim_fov)
        }) {
            let angle_delta = Self::calc_angle(local_player.pos.clone(), target.read().pos);
            (*writable.local_player).view_angles = angle_delta;
        }
        Ok(())
    }
}

impl Aimbot {

    pub fn new() -> Aimbot{
        Aimbot { last_target: std::ptr::null_mut::<Ent>(), aim_fov: DEFAULT_AIM_FOV}

    }


    pub fn calc_angle(root: Pos, target: Pos) -> ViewAngles {
        let delta = Pos {
            x: root.x - target.x,
            y: root.y - target.y,
            z: root.z - target.z,
        };

        let hyp = f32::sqrt(delta.x.powi(2) + delta.y.powi(2) + delta.z.powi(2));

        let yaw = f32::atan2(delta.y, delta.x) * 180f32 / PI + 270f32;
        let pitch = f32::acos(delta.z / hyp) * 180f32 / PI - 90f32;
        

        ViewAngles { pitch, yaw }
    }

}
