#![feature(strict_provenance)]

use std::{
    thread, fs::OpenOptions, time::Duration, ffi::{CString, CStr}};
use chrono::Utc;

use ctor::{dtor,ctor};
use null_terminated;

mod logger;
mod process;
mod util;
mod mem;

use libc::c_void;
use logger::Logger;
use util::Result;
use process::Process;
use mem::*;

use crate::mem::nts::to_string;

static mut LOGGER: Option<Logger> = None;

#[ctor]
fn load(){
    thread::spawn(|| {
        unsafe {
            LOGGER = Some(Logger::new().expect("failed to init logger"));
            
        }
        if let Err(e) = main() {
            log!("{}", e);
            unload()
        }

    });
}

const PLAYER: usize = 0x5a3518;
const HP: usize = 0x100;
const NAME: usize = 0x219;
const PLAYER_LIST: usize = 0x5a3520;
const POS_X: usize = 0x8;
const POS_Y: usize = 0xc;
const POS_Z: usize = 0x10;
const PLAYER_COUNT: usize = 0x5a352c;

#[derive(Debug)]
struct Ent {
    name: *mut nts::nts,
    hp: *mut usize,
    x: *mut f32,
    y: *mut f32,
    z: *mut f32,
}

impl Ent {
    pub fn load(address: usize) -> Ent{
        Ent{
            name: follow_offsets::<nts::nts>(address+NAME, []),
            hp: follow_offsets::<usize>(address+HP, []),
            x: follow_offsets::<f32>(address+POS_X, []),
            y: follow_offsets::<f32>(address+POS_Y, []),
            z: follow_offsets::<f32>(address+POS_Z, []),
        }
    }
}

fn main() -> Result<()>{

    let proc = Process::this()?;
    let local_player_ptr = follow_offsets::<usize>(PLAYER_LIST, []);
    unsafe{
        let ent = Ent::load(*local_player_ptr);
        log!("{:?}<{:?}>: {:?},{:?},{:?}",(*(ent.name)).as_string()?, *(ent.hp) , *(ent.x), *(ent.y), *(ent.z));
    }

    loop {

        let players = follow_offsets::<[usize;3]>(PLAYER_LIST, [0x8]);
        unsafe{
            for player in *players {
                let ent = Ent::load(player);
                log!("{:?}<{:?}>: {:?},{:?},{:?}",(*(ent.name)).as_string()?, *(ent.hp) , *(ent.x), *(ent.y), *(ent.z));
            }
        }

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

