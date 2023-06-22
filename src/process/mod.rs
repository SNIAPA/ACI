use std::{collections::HashMap, process, fs::read_to_string};

pub mod module;

use self::module::{Module, BaseModule};
use crate::util::Result;

pub struct Process {
    pid: u32,
    pub dir: String,
    pub modules: HashMap<String, Module>,
}

impl Process {
    pub fn this() -> Result<Self> {
        let pid = process::id();
        let dir = format!("/proc/{}", pid);
        let modules = Process::load_modules(&dir)?;



        Ok(Process { pid, dir, modules })
    }
    
    pub fn load_modules(dir: &str) -> Result<HashMap<String,Module>> {

        let maps = read_to_string(format!("{dir}/maps"))?;

        let modules = maps.lines()
            .filter_map(|line| {
                Module::load_from_maps_line(line).ok()
             })
             .map(|module| {
                (module.get_name(),module)
            })
            .collect();
        Ok(modules)
    }
}
