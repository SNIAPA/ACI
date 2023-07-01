#![feature(strict_provenance)]

use std::{thread, time::Duration, cmp::Ordering, sync::{Arc, Mutex, RwLock}, collections::HashMap};

use cheat::{Cheat, aimbot::Aimbot, esp::Esp, CheatModule};
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


pub fn run(modules:& HashMap<String, Box<dyn CheatModule>>, writable: &mut Cheat) -> Result<()>{
    for module in modules.values() {
        unsafe{
            let _ = module.run(writable);
        }
    }
    Ok(())
}

fn main() -> Result<()>{

    let mut cheat_wraped = Arc::new(RwLock::new(Cheat::init()));
    let aimbot = Aimbot::new();
    let esp = Esp::new(cheat_wraped.clone());
    let modules: HashMap<String, Box<dyn CheatModule>> = HashMap::from([
        ("aimbot".to_owned(), Box::new(aimbot) as _),
        ("esp".to_owned(), Box::new(esp) as _),
    ]);

    loop {
        unsafe{
            thread::sleep(Duration::from_millis(1));
            run(&modules, &mut cheat_wraped.write().unwrap())?;
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

