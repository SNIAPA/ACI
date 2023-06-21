use crate::util::{
    Result,
    error::CheatError
};

pub struct Special{
    pub module: BaseModule
}

pub struct Executable{
    pub path: String,
    pub module: BaseModule
}

pub struct BaseModule{
    pub name: String,
    pub offset: usize
}

pub enum Module{
    Special(Special),
    Executable(Executable)
}

impl Module {
    pub fn load(maps_line: &str) -> Result<Module>{

        let mut columns:Vec<String> = maps_line
            .split_whitespace()
            .map(|x|{x.to_string()})
            .collect();

        if columns.len() < 6 {
            return Err(CheatError::new("invalid vmaps len").into());
        }
        if columns.len() > 6 {
            columns[5] = columns[5..].join(" ");
        }

        let mut name = columns[5].to_string();

        let offset = columns[2].split('-').next().expect("test");
        let offset = usize::from_str_radix(offset,16)?; 
        if ["[stack]", "[heap]"].contains(&&name.as_str()) {
            name = name.replace("[", "").replace("]", "");
            return Ok(Module::Special(Special {module: BaseModule {offset, name}}));
        };

        if !columns[1].contains("x") {
            return Err(CheatError::new("cant load module").into());
        }

        let file_name =  name.split("/")
            .last()
            .ok_or("error parsing vmaps")?
            .split(".")
            .next()
            .ok_or("error parsing vmaps")?
            .to_string();

        Ok(Module::Executable(Executable {path: name, module: BaseModule {offset, name: file_name}}))
    }
}
