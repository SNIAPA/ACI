#![feature(strict_provenance)]

use std::{thread, time::Duration, cmp::Ordering};

use ctor::{dtor,ctor};
mod logger;
mod process;
mod util;
mod mem;
mod cheat;
mod structures;

use logger::Logger;
use util::Result;
use structures::ent::*;

use crate::{mem::follow_offsets, structures::ViewMatrix, cheat::calc_angle};


static mut LOGGER: Option<Logger> = None;

#[ctor]
fn load(){
    thread::spawn(|| {
        unsafe {
            LOGGER = Some(Logger::new().expect("failed to init logger"));
        }

        if let Err(e) = std::panic::catch_unwind(main) {
            log!("error: {:?}", e);
            thread::sleep(Duration::from_secs(1));
            unload()
        }

    });
}

const PLAYER: usize = 0x5a3518;
const PLAYER_LIST: usize = 0x5a3520;
const PLAYER_COUNT: usize = 0x5a352c;
const VIEW_MATRIX: usize = 0x59e0c0;

const AIM_FOV:f32 = 10.0;

//MVP: make the ofsets into a statics somehow
fn main() -> Result<()>{

    //let proc = Process::this()?;
    let viewMatrix = follow_offsets::<ViewMatrix>(VIEW_MATRIX,[]);

    thread::spawn(||{
        let player_count = follow_offsets::<u8>(PLAYER_COUNT, []);
        let local_player_ptr = follow_offsets::<Ent>(PLAYER, [0x0]);
        loop {
            unsafe {
                if player_count.read_unaligned() == 0 {
                    continue;
                }

                let players = follow_offsets::<[*mut Ent;255]>(PLAYER_LIST, [0x8]);
                let players = &mut (*players)[..player_count.read_unaligned() as usize -1usize];
                players.sort_by(|&a,&b|{
                    (*a).pos.dist(&(*local_player_ptr).pos)
                        .partial_cmp(&(*b).pos.dist(&(*local_player_ptr).pos)).unwrap()

                });

                if let Some(target) = players.iter().find(|&&x|{
                        let view_angles = calc_angle((*local_player_ptr).pos.clone(), (*x).pos);
                        (*x).hp <= 100 && ((view_angles.yaw - (*local_player_ptr).view_angles.yaw).abs() + (view_angles.pitch - (*local_player_ptr).view_angles.pitch).abs() < AIM_FOV)
                        
                    }) {

                    let angle_delta = calc_angle((*local_player_ptr).pos.clone(), (**target).pos);
                    (*local_player_ptr).view_angles = angle_delta;

                }
            }
            
        }
    });
    let local_player_ptr = follow_offsets::<Ent>(PLAYER, [0x0]);

    let player_count = follow_offsets::<u8>(PLAYER_COUNT, []);
    loop {
        unsafe {
            (*local_player_ptr).hp = 1337;
            (*local_player_ptr).ammo = 1337;
        }


        unsafe{
            if player_count.read_unaligned() == 0 {
                thread::sleep(Duration::from_secs(1));
                continue;
            }
        }
        let players = follow_offsets::<[*mut Ent;255]>(PLAYER_LIST, [0x8]);

        log!("+{:=^78}+","");
        log!("| {:^20} | {:^20} | {:^30} |","name","hp","pos");
        log!("+{:_^78}+","");

        let mut lines: Vec<String> = Vec::new();
        unsafe{
            let players = &mut (*players)[..player_count.read_unaligned() as usize -1usize];


            for player in players {
                lines.push(player.read().to_string());
            }
        }
        log!("{}",lines.join(format!("\n+{:-^78}+\n","").as_str()));
        log!("+{:=^78}+","");

        thread::sleep(Duration::from_secs(1));
    }
 
    #[allow(unreachable_code)]
    Ok(())
}

#[dtor]
fn unload(){ 
    log!("unloading");

   unsafe{
       LOGGER.as_mut().unwrap().console.kill().unwrap();
   }

}

