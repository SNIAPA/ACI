use std::thread;

use ctor::{dtor,ctor};

mod logger;

use logger::Logger;

static mut LOGGER: Option<Logger> = None;

#[ctor]
fn load(){
    thread::spawn(|| {
        unsafe {
            LOGGER = Some(Logger::new().expect("failed to init logger"));
        }
    });
}

fn unload(){ 

    unsafe{
        LOGGER.as_mut().unwrap().console.kill().unwrap();
    }

}

