use std::{io,fs::{File, create_dir, OpenOptions}, io::Write, process::{Child, Command, Stdio}, path::Path, os::unix::prelude::AsRawFd, error::Error};
use chrono::Utc;
use libc::{termios, TCSAFLUSH};

use crate::util::Result;

static LOG_DIR: &str = "/tmp/ACI/";
static INIT_MESSAGE: &str = r#"
░░░░░▄▄▄▄▀▀▀▀▀▀▀▀▄▄▄▄▄▄░░░░░░░░
░░░░░█░░░░▒▒▒▒▒▒▒▒▒▒▒▒░░▀▀▄░░░░
░░░░█░░░▒▒▒▒▒▒░░░░░░░░▒▒▒░░█░░░
░░░█░░░░░░▄██▀▄▄░░░░░▄▄▄░░░░█░░
░▄▀▒▄▄▄▒░█▀▀▀▀▄▄█░░░██▄▄█░░░░█░
█░▒█▒▄░▀▄▄▄▀░░░░░░░░█░░░▒▒▒▒▒░█
█░▒█░█▀▄▄░░░░░█▀░░░░▀▄░░▄▀▀▀▄▒█
░█░▀▄░█▄░█▀▄▄░▀░▀▀░▄▄▀░░░░█░░█░
░░█░░░▀▄▀█▄▄░█▀▀▀▄▄▄▄▀▀█▀██░█░░
░░░█░░░░██░░▀█▄▄▄█▄▄█▄████░█░░░
░░░░█░░░░▀▀▄░█░░░█░█▀██████░█░░
░░░░░▀▄░░░░░▀▀▄▄▄█▄█▄█▄█▄▀░░█░░
░░░░░░░▀▄▄░▒▒▒▒░░░░░░░░░░▒░░░█░
░░░░░░░░░░▀▀▄▄░▒▒▒▒▒▒▒▒▒▒░░░░█░
░░░░░░░░░░░░░░▀▄▄▄▄▄░░░░░░░░█░░
 _____ _                  _          _____         _ _ 
|_   _(_)                | |        |_   _|       | | |
  | |  _ _ __ ___   ___  | |_ ___     | |_ __ ___ | | |
  | | | | '_ ` _ \ / _ \ | __/ _ \    | | '__/ _ \| | |
  | | | | | | | | |  __/ | || (_) |   | | | | (_) | | |
  \_/ |_|_| |_| |_|\___|  \__\___/    \_/_|  \___/|_|_|
"#;

#[macro_export]
macro_rules! log {

    ($($message: expr),*) => {

        unsafe{
            let msg = &format!($($message),*);

            crate::LOGGER
                .as_mut()
                .unwrap()
                .log(&format!("{}",msg))
                .unwrap();
        }
    };
}

#[macro_export]
macro_rules! dbg {

    ($($message: expr),*) => {

        unsafe{
            let msg = &format!($($message),*);
            let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S");
            let line = std::line!();
            let file = std::file!();

            crate::LOGGER
                .as_mut()
                .unwrap()
                .log(&format!("[{}] {}:{}\n {}",timestamp,file,line,msg))
                .unwrap();
        }
    };
}

pub struct Logger{
    current_file: File,
    last_file: File,
    pub console: Child
}

impl Logger {
    pub fn new() -> Result<Logger> {
        let curret_file_name = format!("{}log-{}.log", LOG_DIR, Utc::now().format("%Y-%m-%d %H:%M:%S"));
        let last_file_name = format!("{}log-last.log", LOG_DIR );

        if !(Path::new(LOG_DIR).is_dir()){
            create_dir(LOG_DIR)?;
        }
        let current_file = File::create(&curret_file_name)?;
        let last_file = File::create(&last_file_name)?;
        let path = std::env::var("TERM")?;
        let console = Command::new(path)
            .args(["-e", "tail","-n","100000", "-f", &last_file_name])
            .stdin(Stdio::piped())
            .spawn()?;

        let mut logger = Logger {last_file,current_file, console};
        logger.log(INIT_MESSAGE)?;
        Ok(logger)

    }
    pub fn log(& mut self, message: &str) -> Result<()> {
        self.last_file.write(format!("{}\n",message).as_bytes())?;
        self.current_file.write(format!("{}\n",message).as_bytes())?;
        Ok(())
    }

    
}

