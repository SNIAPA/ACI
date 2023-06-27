#![feature(strict_provenance)]

use std::{thread, time::Duration};

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

use crate::mem::follow_offsets;


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



//MVP: make the ofsets into a statics somehow
fn main() -> Result<()>{

    //let proc = Process::this()?;
    let local_player_ptr = follow_offsets::<Ent>(PLAYER, [0x0]);

    log!("+{:=^78}+","");
    log!("| {:^20} | {:^20} | {:^30} |","name","hp","pos");
    log!("+{:_^78}+","");
    log!("{}", *local_player_ptr);
    log!("+{:=^78}+","");

    let player_count = follow_offsets::<u8>(PLAYER_COUNT, []);

    //INFO: usefull ent debug thig
    #[cfg(debug_assertions)]
    {
        log!("{:?}",*local_player_ptr);
    }


    loop {
        unsafe {
            (*local_player_ptr).hp = 100;
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

