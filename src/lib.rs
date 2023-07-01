#![feature(strict_provenance)]

use std::{thread, time::Duration, cmp::Ordering, sync::{Arc, Mutex, RwLock}};

use cheat::{Cheat, aimbot::Aimbot, esp::Esp};
use ctor::{dtor,ctor};
mod logger;
mod process;
mod util;
mod mem;
mod cheat;
mod structures;
mod offsets;

use logger::Logger;
use util::{Result, error::CheatError};


static mut LOGGER: Option<Logger> = None;

#[ctor]
fn load(){
    thread::spawn(|| {
        unsafe {
            LOGGER = Some(Logger::new().expect("failed to init logger"));
        }


        if let Err(e) = std::panic::catch_unwind(main) {
            log!("error: {:?}", e);
            thread::sleep(Duration::from_secs(100));
            unload()
        }

    });
}



fn main() -> Result<()>{

    let mut cheat_wraped = Arc::new(RwLock::new(Cheat::init()));
    let cheat_clone = cheat_wraped.clone();
    Cheat::load_modules(cheat_clone)?;

    loop {
        unsafe{
            thread::sleep(Duration::from_millis(100));
            Cheat::run(cheat_wraped.clone())?;
        }
         
    }

    Ok(())
}

#[dtor]
fn unload(){ 
    log!("unloading");

   unsafe{
       LOGGER.as_mut().unwrap().console.kill().unwrap();
   }

}

