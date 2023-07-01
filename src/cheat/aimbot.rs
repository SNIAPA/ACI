use super::{Cheat, CheatModule};
use crate::{structures::{ent::Ent, Pos, ViewAngles}, util::{Result, error::CheatError}, mem::follow_offsets, offsets::PLAYER_COUNT};
use std::{f32::consts::PI, sync::{Mutex, Arc, RwLock}, thread, time::Duration};

static DEFAULT_AIM_FOV: f32 = 10.0;

pub struct Aimbot {
    last_target: *mut Ent,
    aim_fov: f32,
    cheat: Arc<RwLock<Cheat>>,
}

impl CheatModule for Aimbot {
    unsafe fn run(&self) -> Result<()>{
        

        let player_count = self.cheat.read().unwrap().player_count;

        if *player_count == 0 {
            return Ok(())
        };

        let players = &mut self.cheat.read().unwrap().player_list.read()[..(player_count.read()-1) as usize] as *mut _; 
        let local_player = self.cheat.read().unwrap().local_player.read();
        let players = players as *mut [*mut Ent];
        println!("{:?}",*players);

        return Ok(());
    }
}

impl Aimbot {

    pub fn new(cheat: Arc<RwLock<Cheat>>) -> Aimbot{
        Aimbot { last_target: std::ptr::null_mut::<Ent>(), aim_fov: DEFAULT_AIM_FOV, cheat}

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
