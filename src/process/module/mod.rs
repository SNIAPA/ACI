use libloading;

use crate::util::{
    Result,
    error::CheatError
};

pub struct Special{
    pub name: String,
    pub offset: usize
}

pub struct Executable{
    pub path: String,
    pub name: String,
    pub offset: usize
}

pub trait BaseModule {
    fn get_offset(&self) -> usize;
    fn get_name(&self) -> String;
}

impl BaseModule for Module {
    fn get_offset(&self) -> usize {
        match self {
            Module::Special(x) => x.offset,
            Module::Executable(x) => x.offset
        }
        
    }
    fn get_name(&self) -> String {
        match self {
            Module::Special(x) => x.name.clone(),
            Module::Executable(x) => x.name.clone()
        }
    }
}

pub enum Module{
    Special(Special),
    Executable(Executable)
}

impl Module {
    pub fn load_from_maps_line(maps_line: &str) -> Result<Module>{

        let mut columns:Vec<String> = maps_line
            .split_whitespace()
            .map(|x|{x.to_string()})
            .collect();

        if columns.len() < 6 {
            return Err(CheatError::new("invalid vmaps len".to_owned()).into());
        }
        if columns.len() > 6 {
            columns[5] = columns[5..].join(" ");
        }

        let mut name = columns[5].to_string();

        let offset = columns[2].split('-').next().expect("test");
        let offset = usize::from_str_radix(offset,16)?; 
        if ["[stack]", "[heap]"].contains(&&name.as_str()) {
            name = name.replace("[", "").replace("]", "");
            return Ok(Module::Special(Special {offset, name}));
        };

        if !columns[1].contains("x") {
            return Err(CheatError::new("cant load module".to_owned()).into());
        }

        let file_name =  name.split("/")
            .last()
            .ok_or("error parsing vmaps")?
            .split(".")
            .next()
            .ok_or("error parsing vmaps")?
            .to_string();
        

        Ok(Module::Executable(Executable {path: name, offset, name: file_name}))
    }
}
