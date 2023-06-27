use std::mem::size_of;
use core::fmt;

use crate::mem::nts::Nts;

#[repr(C)]
#[derive(Debug)]
pub struct  Ent {
    pub _pad1: [u8; 0x8],
    pub pos: [f32; 3], //0x8
    pub _pad2: [u8; 0x38 - 0x8 - size_of::<[f32; 3]>()],
    pub angle: [f32; 2], //0x38
    pub _pad3: [u8; 0x100 - 0x38 - size_of::<[f32; 2]>()],
    pub hp: u32, //0x100
    pub _pad4: [u8; 0x154 - 0x100 - size_of::<u32>()],
    pub ammo: u32, //0x154
    pub _pad5: [u8; 0x219 - 0x154 - size_of::<u32>()],
    pub name: Nts, //0x219
}

impl fmt::Display for Ent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hp = if self.hp >= 100 { self.hp.to_string() } else { "DEAD".to_string() };

        let pos = self.pos.iter()
            .map(|x|{x.to_string()})
            .collect::<Vec<String>>()
            .join(",");
        write!(f,"| {:^20} | {:^20} | {:^30} |", self.name.to_string(), hp, pos)
    }
}
