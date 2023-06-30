use super::{Cheat, CheatModule};
use crate::structures::{ent::Ent, Pos, ViewAngles};
use std::f32::consts::PI;

static DEFAULT_AIM_FOV: f32 = 10.0;

pub struct Aimbot {
    last_target: *mut Ent,
    aim_fov: f32,
    cheat: *mut Cheat,
}

impl CheatModule for Aimbot {
    fn cheat(&self) -> *mut Cheat {
        self.cheat
    }
    unsafe fn run(&self) {
        println!("AIMBOT");
        
        let player_count = self.cheat.clone().read().player_count;

        println!("{}", *player_count);

        return;
        if *player_count < 2 {return};

        let mut players = self.cheat.read().player_list.read();
        let local_player = self.cheat.read().local_player.read();

        players.sort_by(|&a, &b| {
            a.read()
                .pos
                .dist(&local_player.pos)
                .partial_cmp(&b.read().pos.dist(&local_player.pos))
                .unwrap()
        });

        if let Some(target) = players.iter().copied().find(|&x| {
            let view_angles = Self::calc_angle(local_player.pos.clone(), x.read().pos);
            (*x).hp <= 100
                && ((view_angles.yaw - local_player.view_angles.yaw).abs()
                    + (view_angles.pitch - local_player.view_angles.pitch).abs()
                    < self.aim_fov)
        }) {
            let angle_delta = Self::calc_angle(local_player.pos.clone(), target.read().pos);
            self.cheat.read().local_player.read().view_angles = angle_delta;
        }
    }
}

impl Aimbot {

    pub fn new(cheat: *mut Cheat) -> Aimbot{
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
