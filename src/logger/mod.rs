use std::{fs::{File, create_dir}, io::Write, process::{Child, Command}, path::Path};
use chrono::Utc;

static LOG_DIR: &str = "/tmp/ACC/";
static INIT_MESSAGE: &str = "░░░░░▄▄▄▄▀▀▀▀▀▀▀▀▄▄▄▄▄▄░░░░░░░
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
░░░░░░░░░░░░░░▀▄▄▄▄▄░░░░░░░░█░░";

macro_rules! log {
    ($message: expr) => {
        unsafe{
            crate::LOGGER.as_mut().unwrap().log($message).unwrap();
        }
    };
}

pub struct Logger{
    file: File,
    pub console: Child
}
impl Logger {
    pub fn new() -> Result<Logger, std::io::Error> {
        let file_name = format!("{}-{}.log", LOG_DIR, Utc::now().format("%s"));

        if !(Path::new(LOG_DIR).is_dir()){
            create_dir(LOG_DIR)?;
        }
        let file = File::create(&file_name)?;
        let console = Command::new("alacritty").args(["-e", "tail","-n","100000", "-f", &file_name]).spawn()?;

        let mut logger = Logger {file, console};
        logger.log(INIT_MESSAGE)?;
        Ok(logger)

    }
    pub fn log(& mut self, message: &str) -> Result<(),std::io::Error> {
        self.file.write(format!("{}\n",message).as_bytes())?;
        Ok(())
    }

    
}
