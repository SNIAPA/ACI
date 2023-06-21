use std::{thread, error::Error, fs::OpenOptions};
use libloading::{self, os::unix::Library};

use ctor::{dtor,ctor};

mod logger;

use logger::Logger;

static mut LOGGER: Option<Logger> = None;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[ctor]
fn load(){
    thread::spawn(|| {
        unsafe {
            LOGGER = Some(Logger::new().expect("failed to init logger"));
            
        }
        if let Err(e) = main() {
            log!("{}",e);
            unload();
            load();
        }

    });
}

fn main() -> Result<()>{


    Library::this().close().unwrap();
    log!("test");

    Ok(())
}

#[dtor]
fn unload(){ 
    log!("unloading");

    unsafe{
        LOGGER.as_mut().unwrap().console.kill().unwrap();
    }

}

