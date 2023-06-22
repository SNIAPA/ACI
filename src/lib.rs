use std::{thread, fs::OpenOptions, os::unix::prelude::FileExt, time::Duration};
use chrono::Utc;

use ctor::{dtor,ctor};

mod logger;
mod process;
mod util;

use logger::Logger;
use util::Result;
use process::Process;

use crate::process::module::{BaseModule, Executable, Module};

static mut LOGGER: Option<Logger> = None;


#[ctor]
fn load(){
    thread::spawn(|| {
        unsafe {
            LOGGER = Some(Logger::new().expect("failed to init logger"));
            
        }
        if let Err(e) = main() {
            log!("{}",e);
            unload()
        }

    });
}

fn main() -> Result<()>{

    let proc = Process::this()?;
    log!("MODULES:\n{}", proc
        .modules
        .values()
        .into_iter()
        .map(|module|{
            format!("{} =<> {:#x}", module.get_name(), module.get_offset())
        })
        .collect::<Vec<String>>()
        .join("\n")
    );

    loop {
        if let Module::Executable(client_module) = proc.modules.get("linux_64_client").ok_or("linux_64_client not loaded")? {
            let mem = OpenOptions::new()
                .read(true)
                .write(true)
                .open(format!("{}/mem", proc.dir))?;
            let mut buf = [0u8; 32];
            mem.read_at(&mut buf, client_module.offset as u64 + 0x5a3518 + 0x100)?;
            log!("{:?}", buf);

        }
        thread::sleep(Duration::from_secs(1));
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

