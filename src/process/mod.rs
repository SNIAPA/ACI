use std::{process, fs::{read_to_string, OpenOptions, File}, collections::HashMap};


pub mod module;

use crate::{util::Result, process::module::BaseModule};

use self::module::Module;

#[allow(dead_code)]
pub struct Process {
    pub pid: u32,
    pub dir: String,
    pub modules: HashMap<String, Vec<Module>>,
}

impl Process {
    #[allow(dead_code)]
    pub fn this() -> Result<Self> {
        let pid = process::id();
        let dir = format!("/proc/{}", pid);
        let modules = Process::load_modules(&dir)?;



        Ok(Process { pid, dir, modules })
    }
    
    pub fn load_modules(dir: &str) -> Result<HashMap<String, Vec<Module>>> {

        let maps = read_to_string(format!("{dir}/maps"))?;

        let modules = maps.lines()
            .filter_map(|line| {
                Module::load_from_maps_line(line).ok()
            });
        let mut grouped_modules: HashMap<String, Vec<Module>> = HashMap::new();
        for module in modules {
            if let Some(list) = grouped_modules.get_mut(&module.get_name()) {
                list.push(module)
            } 
            else {
                grouped_modules.insert(module.get_name(), vec![module]);
            }
        }
        Ok(grouped_modules)
    }

    #[allow(dead_code)]
    pub fn get_mem(&self) -> Result<File> {

        Ok(OpenOptions::new()
            .read(true)
            .write(true)
            .open(format!("{}/mem", self.dir))?)
    }
}
