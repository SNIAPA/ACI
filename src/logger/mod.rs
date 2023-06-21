use std::{fs::{File, create_dir, OpenOptions}, io::Write, process::{Child, Command, Stdio}, path::Path};
use chrono::Utc;

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
        use chrono::Utc;

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
    file: File,
    pub console: Child
}
impl Logger {
    pub fn new() -> Result<Logger, std::io::Error> {
        let file_name = format!("{}log_{}.log", LOG_DIR, Utc::now().format("%Y-%m-%d %H:%M:%S"));

        if !(Path::new(LOG_DIR).is_dir()){
            create_dir(LOG_DIR)?;
        }
        let file = File::create(&file_name)?;
        let console = Command::new("alacritty")
            .args(["-e", "tail","-n","100000", "-f", &file_name])
            .spawn()?;

        let mut logger = Logger {file, console};
        logger.log(INIT_MESSAGE)?;
        Ok(logger)

    }
    pub fn log(& mut self, message: &str) -> Result<(),std::io::Error> {
        self.file.write(format!("{}\n",message).as_bytes())?;
        Ok(())
    }

    
}

