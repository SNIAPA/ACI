use std::{
    thread, fs::OpenOptions, time::Duration, ffi::CString};
use chrono::Utc;

use ctor::{dtor,ctor};

mod logger;
mod process;
mod util;
mod mem;

use libc::c_void;
use logger::Logger;
use util::Result;
use process::Process;
use mem::*;

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

fn main() -> Result<()>{

    let proc = Process::this()?;

    loop {
        let mut mem = proc.get_mem()?;

        let player_pointer_addres = 0x5a3518;
        let hp_offset = 0x100;
        let name_offset = 0x219;

        let hp = derefrence_pointer::<u32>(player_pointer_addres, [hp_offset]);
        log!("hp: {:?}", *hp);

        let mut name = derefrence_pointer::<i8>(player_pointer_addres, [name_offset]);
        
        unsafe{
            let name = CString::from_raw(name);
            log!("name: {:?}", name);
        }

        thread::sleep(Duration::from_secs(1));
    }
 
    #[allow(unreachable_code)]
    Ok(())
}

#[dtor]
fn unload(){ 
    log!("unloading");

  //  unsafe{
  //      LOGGER.as_mut().unwrap().console.kill().unwrap();
  //  }

}

