#![feature(strict_provenance)]

use std::{thread, time::Duration, cmp::Ordering};

use cheat::Cheat;
use ctor::{dtor,ctor};
mod logger;
mod process;
mod util;
mod mem;
mod cheat;
mod structures;
mod offsets;

use logger::Logger;
use util::Result;



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



fn main() -> Result<()>{
    let mut cheat = Cheat::init();
    cheat.load_modules();

    loop{
        cheat.run();
        thread::sleep(Duration::from_millis(100))
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

